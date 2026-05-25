mod token;
mod lexer;

use std::fs;


fn main() {
    //Obsidian transpiler!    
    let input_path = "obscode/main.obs";

    let source_code = fs::read_to_string(input_path).expect("Failed to read Obsidian source file!"); 

    let mut lexer = lexer::Lexer {
        source_characters: source_code.chars().collect(),
        cursor_position: 0,
    };

    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

}



