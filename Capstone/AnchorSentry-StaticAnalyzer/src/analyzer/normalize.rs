use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, ItemStruct};
use std::collections::HashMap;
use crate::state::*;



pub fn normalize_program(ast: &syn::File) -> (
    Vec<NormalizedAccountStruct>,
    Vec<ItemStruct>,
    Vec<AccountInstructions>,
    Vec<NormalizedFunctionArgs>,
) {
    let mut account_structs: Vec<NormalizedAccountStruct> = Vec::new();
    let mut storage_structs: Vec<ItemStruct> = Vec::new();

    let mut accounts_with_instructions: Vec<AccountInstructions> = Vec::new();

    let mut functions_args: Vec<NormalizedFunctionArgs> = Vec::new();

    for item in &ast.items {
        if let syn::Item::Struct(s) = item {
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

    (
        account_structs,
        storage_structs,
        accounts_with_instructions,
        functions_args,
    )

    // rules__fn_missing_instruction_args(&accounts_with_instructions, &functions_args, r);

    // // println!("{:?}", account_structs);
    // for a in account_structs {
    //     // println!("{:?}", a);
    //     rules__missing_init_if_needed(&a, r);
    //     rules__wrong_space_assignment(&a, &storage_structs, r);
    //     // for f in a.fields{
    //     //     let space = extract_space(f);
    //     //     for s in &storage_structs {

    //     //     }
    //     // }
    // }
    // r.print();
}

pub fn normalize_struct(s: &syn::ItemStruct) -> NormalizedAccountStruct {
    let normalized_fields: Vec<NormalizedAccountField> = normalize_struct_field(s);
    let normalized_struct = NormalizedAccountStruct {
        name: s.ident.to_string(),
        fields: normalized_fields,
        line: s.ident.span().start().line,
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
        let mut attributes: HashMap<String, AttributeValue> = HashMap::new();

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
                // if attr_str_trimmed.contains("seeds") {
                //     continue;
                // }
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
            line,
        })
    }
    // println!("{:#?}", normalized_fields);
    normalized_fields

    // println!("Attributes: {:#?}", attributes);

    // println!("acct name: {}", name);
    // println!("base type : {}", base_type);
    // println!("generic type : {:?}", generic_args);
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