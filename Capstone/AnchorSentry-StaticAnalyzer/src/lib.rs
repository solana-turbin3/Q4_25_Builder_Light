pub mod state;
pub mod analyzer;
pub mod report;

use std::path::{Path};
use std::fs;
use anyhow::{Context, Ok, Result};
use analyzer::rules::*;
use analyzer::rule_division_by_zero::rules_division_by_variable;
use report::report::*;


use crate::analyzer::normalize_program;

pub fn run_analysis(path: &str) -> Result<()> {

    let path = Path::new(path);

    // let txt_path = Path::new("escrow-ast-raw.txt");

    let ast = parse_rust_file(path)?;

    let mut r = Report::default();
    r.load_file_info(path);


    let (account_structs, 
        storage_structs, 
        accounts_with_instructions, functions_args) = normalize_program(&ast);

    for a in account_structs {
        // println!("{:?}", a);
        rules_missing_init_if_needed(&a, &mut r);
        rules_wrong_space_assignment(&a, &storage_structs, &mut r);
        rules_missing_signer_check(&a, &mut r);
        
    }
    rules_fn_missing_instruction_args(&accounts_with_instructions, &functions_args, &mut r);
    rules_division_by_variable(&ast, &mut r);

    r.print();
    Ok(())
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