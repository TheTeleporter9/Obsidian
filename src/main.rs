mod AST;
mod lexer;
mod parser;
mod tokens;
mod transpile_c;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::transpile_c::transpile_to_c;
use std::fs;

/**
 * ARCHITECTURE NOTE:
 * - Used Gemnini to generate initial pseudocode for the research tree logic.
 * - Manually implemented, refactored, and integrated the logic into this codebase.
 */
fn main() -> std::io::Result<()> {
    let src = fs::read_to_string("main.obs")?;

    let mut lexer = Lexer::new(src);

    println!("Welcome to Obsidian compiler/transpiler");
    println!("Starting lexer");

    lexer.tokenize();

    for token in &lexer.tokens_out {
        println!("{:?}", token);
    }

    println!("*****************************************************************");

    let mut parser = Parser::new(lexer.tokens_out);

    let parser_out = parser.parse();

        for ast_token in &parser_out {
        println!("{:?}", ast_token);
    }

    let c_out: String = transpile_to_c(parser_out.clone());

    fs::write("output.c", c_out)?;  

    Ok(())
}