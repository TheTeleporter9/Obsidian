mod AST;
mod lexer;
mod parser;
mod tokens;

use crate::lexer::Lexer;
use crate::tokens::Tokens;
use crate::parser::Parser;
use std::fs;

fn main() {
    let src = fs::read_to_string("main.obs");

    let mut lexer = Lexer::new(src.unwrap());

    println!("Welcome to Obsidian compiler/transpiler");
    println!("Staring lexer");

    lexer.tokenize();

    for token in &lexer.tokens_out {
        println!("{:?}", token);
    }

    println!("*****************************************************************");

    let mut parser  = Parser::new(lexer.tokens_out);

    parser.parse();

    for ast_token in &parser.tokens {
        println!("{:?}", ast_token);
    }
}
