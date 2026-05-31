use crate::tokens::Tokens;

pub struct Parser {
    current_token_index: usize,
    TOKENS: Vec<Tokens>
}

impl Parser {
    fn parse(&mut self) {
        while self.current_token_index < self.TOKENS.len() {

            if let Some(current_token) = self.TOKENS.get(self.current_token_index) {
                match current_token {
                    Tokens::CONST => print!("parse const variable"),
                    Tokens::VAR => print!("parse vairable token"),
                    Tokens::PRINT => println!("Parse print!"),
                    _ => {}
                }
            }

            self.current_token_index += 1;
        
        }

    }

    fn parse_variable_decleration(&self) {
        let mut variable_identifier_name : usize;

        if let Tokens::Identifier(name) = &self.TOKENS[self.current_token_index + 1] {
            println!("Identifier name: {}", name);
            variable_identifier_name = name.parse().unwrap();
        } else {
            panic!("Syntax Error: Expected vairable name!");
        }

    }
}
