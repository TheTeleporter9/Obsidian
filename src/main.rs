mod lexer;
mod token;
mod ast;
mod parser;

use lexer::Lexer;
use std::fs;
use parser::Parser;


fn main() {
    // 1. Define a piece of test source code using your syntax features
    let source_code = String::from("const damage = 10 + 5 -: calculate()");

    println!("--- Starting Compilation Run ---");
    println!("Source: {}\n", source_code);

    // 2. Initialize the lexer and extract the token vector array
    let mut my_lexer = Lexer::new(source_code);
    let token_stream = my_lexer.tokenize();
    
    println!("Generated Tokens: {:?}", token_stream);
    println!("--------------------------------");

    // 3. Initialize your parser with those tokens
    let mut my_parser = Parser::new(token_stream);
    
    // 4. Run the parser to populate the arena warehouse
    let root_statement_ids = my_parser.parse_program();

    // 5. Print out the root positions returned by the parser execution run
    println!("Root Statement Index IDs: {:?}", root_statement_ids);
    println!("--------------------------------");

    // 6. Print out the inte, current;rnal flat arena vector layout to verify allocations
    println!("Final Flat AST Arena Structure:");
    println!("{:#?}", my_parser.arena);
}