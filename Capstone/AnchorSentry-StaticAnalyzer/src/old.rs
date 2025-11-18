use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, File, ItemStruct, ext, parse_str};
use syn::visit::{self, Visit};
use syn::{Expr, ExprBinary, ExprPath};
// use syn_serde::json;
use std::collections::HashMap;
use std::path::{Path};
use std::fs;
use anyhow::{Context, Ok, Result};
use evalexpr::*;


pub mod state;
pub mod report;
pub mod knowledge_base;
pub mod line_counter;

use crate::knowledge_base::{PossibleDivisionByZeroFinding, *};
use crate::state::{NormalizedAccountField, NormalizedAccountStruct, AttributeValue, NormalizedFunctionArgs, FnArgs, AccountInstructions, PossibleDivisionByZeroFindingCheckerInstance};
use crate::report::Report;
//division

pub struct PossibleDivisionByZeroChecker {
    pub instance: Vec<PossibleDivisionByZeroFindingCheckerInstance>,
    pub current_fn: String
}

impl<'ast> Visit<'ast> for PossibleDivisionByZeroChecker {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.current_fn = node.sig.ident.to_string();

        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr_binary(&mut self, expr: &'ast ExprBinary) {
        // Check if this is a division: left / right
        if matches!(expr.op, syn::BinOp::Div(_)) {
            // expr.right is Box<Expr> -> deref before matching
            let divisor_expr: &Expr = &*expr.right;

            // We only care about the simple case: "some_var" (Expr::Path with single ident)
            if let Expr::Path(ExprPath { path, .. }) = divisor_expr {
                if let Some(ident) = path.get_ident() {
                    // don't attempt to get line/col yet; just include span debug for context
                    let line_number = expr.span().start().line;
                    self.instance.push(PossibleDivisionByZeroFindingCheckerInstance {
                        function_name: self.current_fn.clone(),
                        divisor: ident.to_string(),
                        line: line_number
                    });
                }
            }
        }

        // continue walking inside this binary expression
        visit::visit_expr_binary(self, expr);
    }
}




pub fn parse_rust_file(path: &Path) -> Result<syn::File> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file {}", path.display()))?;

    parse_rust_code(&content).with_context(|| format!("Failed to parse file {}", path.display()))
}

/// Parse a string of Rust code and return the AST
pub fn parse_rust_code(content: &str) -> Result<syn::File> {
    syn::parse_str::<syn::File>(content)
        .map_err(|e| anyhow::anyhow!("Failed to parse Rust code: {}", e))
}

pub fn is_anchor_account_struct(s: &syn::ItemStruct) -> bool {
    if s.attrs.iter().any(|a: &Attribute| a.path().is_ident("derive")){
        //@note: works for now
        if s.attrs.iter().any(|c| c.to_token_stream().to_string().contains("Accounts")) {
            // println!("Finally");
            return true
        }
    }   
    false
}

pub fn is_anchor_storage_struct(s: &syn::ItemStruct) -> bool {
    s.attrs.iter().any(|a: &Attribute| a.path().is_ident("account"))
}

pub fn account_struct_has_instruction(s: &syn::ItemStruct) -> bool {
    if s.attrs.iter().any(|a: &Attribute| a.path().is_ident("instruction")){
        //@note: works for now
        // println!("instruction here: {}", s.ident.to_string());
        return true
    }   
    false
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn anchor_type_size(ty: &str) -> Option<usize> {
    match ty {
        "u8" | "i8" | "bool" => Some(1),
        "u16" | "i16" => Some(2),
        "u32" | "i32" => Some(4),
        "u64" | "i64" => Some(8),
        "u128" | "i128" => Some(16),
        "Pubkey" => Some(32),
        _ => None,
    }
}

pub fn extract_fn_args(fn_item: &syn::ItemFn) -> NormalizedFunctionArgs {
    let mut context_name = String::new();
    let mut args = Vec::new();

    // function name
    let func_name = fn_item.sig.ident.to_string();
    let line  = fn_item.sig.ident.span().start().line;

    for input in &fn_item.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let arg_name = match &*pat_type.pat {
                syn::Pat::Ident(id) => id.ident.to_string(),
                _ => continue,
            };

            if let syn::Type::Path(tp) = &*pat_type.ty {
                let last = tp.path.segments.last().unwrap();
                let ty_ident = last.ident.to_string();

                if ty_ident == "Context" {
                    if let syn::PathArguments::AngleBracketed(args_generic) = &last.arguments {
                        if let Some(syn::GenericArgument::Type(syn::Type::Path(inner))) = args_generic.args.first()
                        {
                            if let Some(seg) = inner.path.segments.last() {
                                context_name = seg.ident.to_string();
                            }
                        }
                    }
                    continue; 
                }

                args.push(FnArgs {
                    name: arg_name,
                    ty: ty_ident,
                });
            }
        }
    }

    NormalizedFunctionArgs {
        name: func_name,
        context: context_name,
        args,
        line
    }
}

pub fn extract_instruction_args(s: &syn::ItemStruct) -> Vec<AccountInstructions> {
    let mut i_vec = Vec::new();

    for attr in &s.attrs {
        if attr.path().is_ident("instruction") {
            let mut out = Vec::new();

            // Parse #[instruction(seed: u64, id: String)]
            if let syn::Meta::List(meta_list) = &attr.meta {
                let raw = meta_list.tokens.to_string();

                for piece in raw.split(',') {
                    let piece = piece.trim();
                    if piece.is_empty() {
                        continue;
                    }
                    let parts: Vec<&str> = piece.split(':').collect();
                    if parts.len() != 2 {
                        continue;
                    }

                    let name = parts[0].trim().to_string();
                    let ty = parts[1].trim().to_string();

                    if !name.is_empty() && !ty.is_empty() {
                        out.push(FnArgs { name, ty });
                    }
                }
            }

            i_vec.push(AccountInstructions {
                ctx_name: s.ident.to_string(),
                args: out
            });
        }
    }
    i_vec

}




pub fn extract_types_from_storage_struct(s: &syn::ItemStruct) -> usize {
    let mut sum = 8; //8 cos were starting with discriminator
    for field in &s.fields {
        // if let syn::Type::Path(type_path) = &field.ty {
        //     println!("{:?}", type_path.path.get_ident().unwrap());
        // }
        if let syn::Type::Path(type_path) = &field.ty {
            if let Some(last_seg) = type_path.path.segments.last() {
                let num = anchor_type_size(&last_seg.ident.to_string()).unwrap();
                sum += num;
                // println!("{:?}", anchor_type_size(&last_seg.ident.to_string()).unwrap());
                // print_type_of(&last_seg.ident);
            }
        }
    }
    // println!("{:?}",sum);
    sum
}

pub fn normalize_program(ast: &syn::File, r: &mut Report) -> Result<()> {
    let mut account_structs: Vec<NormalizedAccountStruct> = Vec::new();
    let mut storage_structs: Vec<ItemStruct> = Vec::new();
    
    let mut accounts_with_instructions: Vec<AccountInstructions> = Vec::new();

    let mut functions_args: Vec<NormalizedFunctionArgs> = Vec::new();

    for item in &ast.items {
        if let syn::Item::Struct(s)= item {
            if is_anchor_account_struct(&s) {
                // normalize_struct_field(s);

                let normalized = normalize_struct(&s);
                // println!("{:?}", normalized);
                account_structs.push(normalized);
            }

            if account_struct_has_instruction(s) {
                // println!("Instruction Args : {:?}", extract_instruction_args(s));
                accounts_with_instructions = extract_instruction_args(s);
            }
        }

        if let syn::Item::Struct(s) = item {
            if is_anchor_storage_struct(&s) {
                storage_structs.push(s.clone());
            }
        }



        if let syn::Item::Mod(module) = item {
            if let Some((_, mod_items)) = &module.content {
                for inner in mod_items {
                    if let syn::Item::Fn(func) = inner {
                        // println!("Found function: {}", func.sig.ident);
                        // println!("{:?}", extract_fn_args(func))
                        functions_args.push(extract_fn_args(func));
                    }
                }
            }
        }

    }

    rules__fn_missing_instruction_args(&accounts_with_instructions, &functions_args, r);
    

    // println!("{:?}", account_structs);
    for a in account_structs {
        // println!("{:?}", a);
        rules__missing_init_if_needed(&a, r);
        rules__wrong_space_assignment(&a, &storage_structs, r);
        // for f in a.fields{
        //     let space = extract_space(f);
        //     for s in &storage_structs {
                
        //     }
        // }
    }
    // r.print();
    Ok(())
}

pub fn normalize_struct(s: &syn::ItemStruct) -> NormalizedAccountStruct {
    let normalized_fields: Vec<NormalizedAccountField> = normalize_struct_field(s);
    let normalized_struct = NormalizedAccountStruct {
        name: s.ident.to_string(),
        fields: normalized_fields,
        line: s.ident.span().start().line
    };
    normalized_struct
}

pub fn normalize_struct_field(s: &syn::ItemStruct) -> Vec<NormalizedAccountField> {
    let mut normalized_fields: Vec<NormalizedAccountField> = Vec::new();

    for field in &s.fields {
        let ctx = s.ident.to_string();
        // let name = field.ident. to_string();
        let name = field.ident.as_ref().map(|i| i.to_string()).unwrap();
        // let base_type = &field.ty. path.segments.first()
        let line = field.ident.span().start().line;

        //base type : Account
        //generic type : ["'info", "TokenAccount"]
        let mut base_type = String::new();
        let mut generic_args = Vec::new();

        if let syn::Type::Path(type_path) = &field.ty {
            // Base type name
            if let Some(first_segment) = type_path.path.segments.first() {
                base_type = first_segment.ident.to_string();

                // Generic arguments, e.g. <'info, TokenAccount>
                if let syn::PathArguments::AngleBracketed(args) = &first_segment.arguments {
                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Lifetime(lt) => {
                                generic_args.push(format!("'{}", lt.ident));
                            }
                            syn::GenericArgument::Type(ty) => {
                                // Nested type, like TokenAccount
                                if let syn::Type::Path(inner_path) = ty {
                                    if let Some(seg) = inner_path.path.segments.last() {
                                        generic_args.push(seg.ident.to_string());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // attribute: seed = [*]
        let mut attributes: HashMap <String, AttributeValue>= HashMap::new();

        for attr in &field.attrs {

            // println!("{}",attr.to_token_stream().to_string());
            // if attr.to_token_stream().is_empty() {
            //     println!("empty")
            // }

            let attr_str = attr.to_token_stream().to_string();
            if attr_str.starts_with("# [account") || attr_str.starts_with("# [accounts") {
                // filter `# [accounts (` or `# [account (`
                let attr_str_trimmed = attr_str
                    .trim_start_matches("# [accounts (")
                    .trim_start_matches("# [account (")
                    .trim_end_matches(")]")
                    .trim();
                //too stressful for now
                if attr_str_trimmed.contains("seeds"){
                    continue;
                }
                for token in attr_str_trimmed.split(',') {
                    let parts: Vec<&str> = token.split('=').collect();

                    if parts.len() == 2 {
                        let key = parts[0].trim().to_string();
                        let value = parts[1].trim().to_string();
                        attributes.insert(key, AttributeValue::String(value));
                    } else {
                        let key = parts[0].trim().to_string();
                        attributes.insert(key, AttributeValue::Bool(true));
                    }
                }
            }
        }
        normalized_fields.push(NormalizedAccountField {
            context: ctx,
            name,
            base_type,
            generic_args,
            attributes,
            line
        })

    }
    // println!("{:#?}", normalized_fields);
    normalized_fields
        
        
        // println!("Attributes: {:#?}", attributes);

        



        // println!("acct name: {}", name);
        // println!("base type : {}", base_type);
        // println!("generic type : {:?}", generic_args);

}


//✅
pub fn rules__missing_init_if_needed(s: &NormalizedAccountStruct, r: &mut Report) {
    // println!("Here");
    for field in &s.fields {
        if field.contains_attr("associated_token") {
            // println!("Probably Token");
            if field.has_bool_attribute("init") {
                // println!("Bug here");
                r.add(Finding::MissingInitIfNeeded(MissingInitIfNeededFinding {
                    rule: &RULE_MISSING_INIT_IF_NEEDED,
                    account: field.name.clone(),
                    line: field.line,
                    context: s.name.clone()
                    
                }));
            }
        }
    }
}

//✅
pub fn rules__wrong_space_assignment(s: &NormalizedAccountStruct, v: &Vec<ItemStruct>, r: &mut Report) {
    for field in &s.fields {
        //for formatting remember to add prints liek checking for space err in field.ident...
        if field.contains_attr("space") {
            let account_space = extract_space(&field);
            let account_data_type = field.generic_args.get(1);
            println!("{:?}", account_data_type.unwrap());

            for item in v {
                if item.ident.to_string() == *account_data_type.unwrap() {
                    let expected_space = extract_types_from_storage_struct(item);
                    if compare_usize_and_evalexpr_value(expected_space, account_space.clone().unwrap()) {
                        println!("No Issue")
                    }else {
                        // println!("Issue");
                        r.add(Finding::WrongSpaceAssignment(WrongSpaceAssignmentFinding {
                            rule: &RULE_WRONG_SPACE_ASSIGNMENT,
                            account: field.name.clone(),
                            line: field.line,
                            expected: expected_space,
                            actual: account_space.clone().unwrap().to_string(),
                        }));
                    }
                }
            }
        }
        
    }
}

pub fn rules__fn_missing_instruction_args(v: &Vec<AccountInstructions>, f: &Vec<NormalizedFunctionArgs>, r: &mut Report) {
    for field in v {
        let name = field.ctx_name.clone();
        for func in f {
            if func.context == name {
                println!("fn and ins match");
                for required_arg in &field.args {
                let exists = func.args.iter().any(|arg|arg.name == required_arg.name && arg.ty == required_arg.ty);

                if !exists {
                    // println!(
                    //     "❌ Missing required instruction argument in function: `{}`: {}: {}",
                    //     func.name, required_arg.name, required_arg.ty);
                        r.add(Finding::MissingRequiredInstructionArgument(MissingRequiredInstructionArgumentFinding {
                            rule: &RULE_MISSING_REQUIRED_INSTRUCTION_ARGUMENT,
                            fn_name : func.name.clone(),
                            line: func.line,
                            required_arg_name: required_arg.name.clone(),
                            required_arg_type: required_arg.ty.clone()
                        }));
                }
            }
            }

        }
    }
}


pub fn extract_space(f: &NormalizedAccountField) -> Option<Value> {
    let s= f.has_str_attribute("space").unwrap_or_default();
    // println!("{:?}", s);
    if !s.is_empty() {
        let result = eval(s).unwrap();
        println!("Result: {:?}", result);
        return Some(result)
    }
    None
}

fn compare_usize_and_evalexpr_value(usize_val: usize, eval_val: evalexpr::Value) -> bool {
    if let evalexpr::Value::Int(n) = eval_val {
        n as usize == usize_val
    } else {
        false
    }
}

pub fn rules__division_by_variable(ast: &syn::File, r: &mut Report) {
    let mut checker = PossibleDivisionByZeroChecker { instance: Vec::new(), current_fn: String::new(), };
    checker.visit_file(ast);
    for p in checker.instance {
        // println!("Possible division by 0 : {:?}", p);
        r.add(Finding::PossibleDivisionByZero(PossibleDivisionByZeroFinding {
            rule: &RULE_POSSIBLE_DIVISION_BY_ZERO,
            fn_name : p.function_name,
            line: p.line,
            divisor: p.divisor       
        }))
    }
}

pub fn main() -> Result<()>{
    let path = Path::new("src/make.rs");

    let txt_path = Path::new("escrow-ast-raw.txt");

    let ast = parse_rust_file(path)?;

    let mut report = Report::default();
    report.load_file_info(path);

    normalize_program(&ast, &mut report);

    rules__division_by_variable(&ast, &mut report);

    report.print();

    // let json_string = syn_serde::json::to_string_pretty(&ast);
    // // json_string;


    // let txt = format!("{:#?}", ast);
    // fs::write(txt_path, txt)
    //     .with_context(|| format!("Failed to write AST to {}", txt_path.display()));

    // println!("{:#?}", ast);
    // for item in &ast.items {
    //     if let syn::Item::Struct(s) = item {
    //         if is_anchor_account_struct(s) {
    //             println!("Account Found: {}", s.ident)
    //         }
    //         if is_anchor_storage_struct(s) {
    //             println!("Storage Account Found: {}", s.ident);
    //             extract_types_from_storage_struct(s);
    //         }
    //     }
        // if let syn::Item::Mod(module) = item {
        //     if module.attrs.iter().any(|a| a.path().is_ident("program")) {
        //         if let Some((_, items)) = &module.content {
        //             for subitem in items {
        //                 if let syn::Item::Fn(func) = subitem {
        //                     println!("Found program function: {}", func.sig.ident);
        //                 }
        //             }
        //         }
        //     }
        // }
    // }

    // for item in &ast.items {
    //     if let syn::Item::Struct(module) = item {
    //         if module.attrs.iter().any(|f| f.path().is_ident("account")) {
    //             println!("Found accounts: {}", module.ident.to_string());
    //         }
    //     }
    // }



    Ok(())
}

