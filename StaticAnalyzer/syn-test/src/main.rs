use anchor_syn::parser::context;
use quote::ToTokens;
// use anchor_syn::parser::{AccountsStructParser, InstructionFnParser, ProgramParser};
// use anchor_syn::parser::program::parse
use syn::{Attribute, File, parse_str};
use syn_serde::json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context, Ok, Result};

pub mod state;

use crate::state::{NormalizedAccountField, NormalizedAccountStruct, AttributeValue};

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
            println!("Finally");
            return true
        }
    }   
    false
}

pub fn normalize_program(ast: &syn::File) -> Result<()> {
    let mut account_structs: Vec<NormalizedAccountStruct> = Vec::new();
    for item in &ast.items {
        if let syn::Item::Struct(s)= item {
            if is_anchor_account_struct(&s) {
                normalize_struct_field(s);

                // let normalized = normalize_struct(&s);
                // account_structs.push(normalized);
            }
        }
    }
    Ok(())
}

pub fn normalize_struct(s: &syn::ItemStruct) -> Result<()> {
    let normalized = NormalizedAccountStruct {
        name: s.ident.to_string(),
        fields: vec![],
    };
    Ok(())
}

// pub fn normalize_field(field: &syn::Field, context_name: &str) -> Result<NormalizedAccountField> {
//     // === 1️⃣ Extract raw data from syn::Field ===
//     let name = field.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
//     let span = field.ident.as_ref().map(|i| {
//         let start = i.span().start();
//         (start.line, start.column)
//     });

//     let type_string = quote::quote!(#field.ty).to_string();
//     let mut base_type = String::new();
//     let mut generic_args = Vec::new();

//     // parse type like Account<'info, TokenAccount>
//     if let Some(start) = type_string.find('<') {
//         base_type = type_string[..start].trim().to_string();
//         let generics_part = &type_string[start + 1..type_string.len() - 1];
//         generic_args = generics_part
//             .split(',')
//             .map(|s| s.trim().to_string())
//             .collect();
//     } else {
//         base_type = type_string;
//     }

//     // === 2️⃣ Extract attributes ===
//     let mut attributes = HashMap::new();
//     let mut seeds = None;
//     let mut space = None;
//     let mut bump = None;
//     let mut payer = None;

//     for attr in &field.attrs {
//         let attr_str = attr.to_token_stream().to_string();
//         if attr_str.contains("account") {
//             for token in attr_str.split(',') {
//                 let parts: Vec<&str> = token.split('=').collect();
//                 if parts.len() == 2 {
//                     let key = parts[0].trim().to_string();
//                     let value = parts[1].trim().to_string();
//                     attributes.insert(key.clone(), AttributeValue::String(value.clone()));

//                     match key.as_str() {
//                         "space" => space = Some(value),
//                         "payer" => payer = Some(value),
//                         "bump" => bump = Some(value),
//                         "seeds" => seeds = Some(vec![value]),
//                         _ => {}
//                     }
//                 } else {
//                     let key = parts[0].trim().to_string();
//                     attributes.insert(key.clone(), AttributeValue::Bool(true));
//                     if key == "bump" {
//                         bump = Some(String::from("true"));
//                     }
//                 }
//             }
//         }
//     }

//     // === 3️⃣ Build struct once ===
//     Ok(NormalizedAccountField {
//         context: context_name.to_string(),
//         name,
//         base_type,
//         generic_args,
//         is_mut: false,
//         attributes,
//         seeds,
//         space,
//         bump,
//         authority: None,
//         payer,
//         mint: None,
//         associated_to: None,
//         span,
//     })
// }


pub fn normalize_struct_field(s: &syn::ItemStruct) -> Result<Vec<NormalizedAccountField>> {
    let mut normalized_fields: Vec<NormalizedAccountField> = Vec::new();

    for field in &s.fields {
        let ctx = s.ident.to_string();
        // let name = field.ident. to_string();
        let name = field.ident.as_ref().map(|i| i.to_string()).unwrap();
        // let base_type = &field.ty. path.segments.first()

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
        })

    }
    println!("{:#?}", normalized_fields);
    Ok(normalized_fields)
        
        
        // println!("Attributes: {:#?}", attributes);

        



        // println!("acct name: {}", name);
        // println!("base type : {}", base_type);
        // println!("generic type : {:?}", generic_args);

}

// pub fn is_anchor_account_struct(s: &syn::ItemStruct) -> bool {
//     if s.attrs.iter().any(|a| a.path().is_ident("derive")){
//         return true        
//     }
//     false
// }

pub fn main() -> Result<()>{
    let path = Path::new("src/make.rs");

    let txt_path = Path::new("escrow-ast-raw.txt");

    let ast = parse_rust_file(path)?;

    normalize_program(&ast);

    // let json_string = syn_serde::json::to_string_pretty(&ast);
    // // json_string;


    // let txt = format!("{:#?}", ast);
    // fs::write(txt_path, txt)
    //     .with_context(|| format!("Failed to write AST to {}", txt_path.display()));

    // println!("{:#?}", ast);
    for item in &ast.items {
        if let syn::Item::Struct(s) = item {
            if is_anchor_account_struct(s) {
                println!("Account Found: {}", s.ident)
            }
        }
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
    }

    // for item in &ast.items {
    //     if let syn::Item::Struct(module) = item {
    //         if module.attrs.iter().any(|f| f.path().is_ident("account")) {
    //             println!("Found accounts: {}", module.ident.to_string());
    //         }
    //     }
    // }



    Ok(())
}

