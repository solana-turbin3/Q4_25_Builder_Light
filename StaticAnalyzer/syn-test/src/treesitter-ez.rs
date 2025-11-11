pub mod line_counter;
use tree_sitter::{InputEdit, Language, Parser, Point, Query, QueryCursor};



// use crate::line_counter::count_lines;

// fn main() {
//     line_counter::count_lines()
// }
fn walk(node: tree_sitter::Node, source: &str) {
    if node.is_named() {
        println!("Node kind: {}", node.kind());
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            walk(child, source);
        }
    }
}


fn main() {

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");

    let source = r#"
        fn add(a: i32, b: i32) -> i32 { a + b }
        fn greet(name: &str) { println!("hi {}", name); }
    "#;

    let tree = parser.parse(source, None).unwrap();

    let query_source = "(function_item name: (identifier) @fn_name)";
    let query = Query::new(&tree_sitter_rust::LANGUAGE.into(), query_source).unwrap();

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    for m in matches.into() {
        for cap in m.captures.iter() {
            let range = cap.node.byte_range();
            let text = &source[range.start..range.end];
            println!("Found function: {}", text);
        }
    }
}

    // Modern version: import directly




// S-expression:
// (source_file (function_item name: (identifier) parameters: (parameters (parameter pattern: (identifier) type: (reference_type type: (primitive_type)))) body: (block (expression_statement (macro_invocation macro: (identifier) (token_tree (string_literal (string_content)) (identifier)))))))