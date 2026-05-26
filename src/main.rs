pub mod ast;
pub mod code_generator;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod typecherker;

use std::env; // Used to read command-line arguments
use std::fs; // Used to read files from disk
use std::process; // Used to exit cleanly on errors

use code_generator::CodeGenerator;
use lexer::Lexer;
use parser::Parser;
use typecherker::TypeChecker;

fn main() {
    // 1. Collect arguments passed via the terminal command line
    let args: Vec<String> = env::args().collect();

    // The first argument (args[0]) is always the compiler binary name itself.
    // The second argument (args[1]) should be our file path.
    if args.len() < 2 {
        println!("Usage Error: Please provide a source file path.");
        println!("Example: cargo run path/to/file.obs");
        process::exit(1);
    }

    let file_path = &args[1];

    // 2. Load the contents of the .obs file into a string
    println!("Loading source file: {} ...", file_path);
    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            println!(
                "File Error: Could not read file '{}' ({})",
                file_path, error
            );
            process::exit(1);
        }
    };

    println!("--- Starting Obsidian Compilation Pipeline ---\n");

    // Phase 1: Lexical Analysis (Tokens)
    let mut my_lexer = Lexer::new(source_code);
    let token_stream = my_lexer.tokenize();

    // Phase 2: Parsing (Syntax Construction)
    let mut my_parser = Parser::new(token_stream);
    let root_statement_ids = my_parser.parse_program();

    // Phase 3: Semantic Analysis (Type Safety Verification)
    println!("Running Type Checker...");
    let mut checker = TypeChecker::new(&my_parser.arena);
    checker.check_program(&root_statement_ids);
    println!("Type check passed smoothly!\n");

    // Phase 4: Code Generation (Transpiling to C)
    println!("Generating Target C Source Code...");
    let generator = CodeGenerator::new(&my_parser.arena);
    let final_c_output = generator.generate_program(&root_statement_ids);

    println!("----------------------------------------------");
    println!("EMITTED C CODE OUTPUT:\n");
    println!("{}", final_c_output);
    println!("----------------------------------------------");

    // Optional Step 5: Save the output string as an actual .c file on disk
    let output_path = "output.c";
    if let Err(e) = fs::write(output_path, final_c_output) {
        println!("Warning: Could not save compiled C file to disk: {}", e);
    } else {
        println!(
            "Successfully compiled and wrote output to '{}'!",
            output_path
        );
    }
}
