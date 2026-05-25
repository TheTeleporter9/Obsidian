mod lexer;
mod token;
mod arena;

use lexer::Lexer;
use std::fs;


fn main() {
    //Obsidian transpiler!
    let input_path = "obscode/main.obs";

    let source_code = fs::read_to_string(input_path).expect("Failed to read Obsidian source file!");

    let mut lexer = Lexer::new(source_code);

    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
