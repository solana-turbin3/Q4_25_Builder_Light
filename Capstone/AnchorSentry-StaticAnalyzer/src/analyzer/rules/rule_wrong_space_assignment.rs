use evalexpr::*;
use syn::{ItemStruct};
use crate::report::report::*;
use crate::report::knowledge_base::{WrongSpaceAssignmentFinding, Finding, RULE_WRONG_SPACE_ASSIGNMENT};
use crate::state::{NormalizedAccountField, NormalizedAccountStruct};


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

fn compare_usize_and_evalexpr_value(usize_val: usize, eval_val: evalexpr::Value) -> bool {
    if let evalexpr::Value::Int(n) = eval_val {
        n as usize == usize_val
    } else {
        false
    }
}

pub fn rules_wrong_space_assignment(s: &NormalizedAccountStruct, v: &Vec<ItemStruct>, r: &mut Report) {
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