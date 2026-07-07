use std::fmt::Binary;

use crate::{AST::{self, ASTNode}, tokens::Tokens::{self, Identifier}};
use crate::DataType;
#[ derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Tokens>,
    current: usize
}


impl Parser {
    
    pub fn new(input_tokens: Vec<Tokens>) -> Self {
        Self {
            tokens: input_tokens,
            current: 0
        }
    }

    pub fn parse_program(&mut self) {
        while self.current <= self.tokens.len() && self.tokens[self.current] != Tokens::EOF {
            match(self.tokens[self.current]) {
                Tokens::EOF => self.parse_end_of_file(),
                Tokens::VAR => self.advance(),
                _ => panic!("Unknown Token!")
            }
        }
    }

    //helper functions

    fn peek(self) -> Tokens {
        if self.current + 1 < self.tokens.len() {
            return self.tokens[self.current + 1].clone();
        }
        return self.tokens[self.current].clone();
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn previous(&mut self) -> Tokens{
        if self.current - 1 > 0 {
            return self.tokens[self.current - 1].clone();
        } else {
            return self.tokens[self.current].clone();
        }
    }

    fn is_at_end(self) -> bool{
        return self.current >= self.tokens.len()
    }

    fn check(&self, token: Tokens) -> bool {
       if token == self.tokens[self.current] {
        return true
       } else {
        return false
       }
    }

    fn advance_if_token(&mut self, token: Tokens) {
        if token == self.tokens[self.current] {
            self.advance();
        }         
    }

    fn consume(&mut self, token: Tokens, error_message : &str){
        if self.check(token) {
            self.advance();
        } else {
            panic!("{}",error_message)
        }
    }


    //End of helper funcitons
}

impl Parser {

    fn parse_expression(&mut self) {
        if self.check(Tokens::OperatorAdd) {

        }
    }

    fn parse_addition_and_subtraction_expression(&mut self) -> ASTNode{
        let mut left = self.parse_multiplication_and_divide_expression();

        let mut operator: Tokens = self.tokens[self.current];

        self.advance();

        let mut right = self.parse_multiplication_and_divide_expression();



    }

    fn parse_multiplication_and_divide_expression(&mut self) {

    }


    fn parse_variable_declaration(&mut self) -> ASTNode{
        self.consume(Tokens::VAR, "Variable declaration not correctly setup");

        let var_type = match &self.tokens[self.current] {
            Tokens::TypeInt => DataType::VarType::Int,
            Tokens::TypeFloat => DataType::VarType::Float,
            Tokens::TypeBoolean => DataType::VarType::Bool,
            _ => panic!("invalid type"),
        };

        self.advance();

        let var_name = match &self.tokens[self.current] {
            Tokens::Identifier(name) => name.clone(),
            _ => panic!("Expected a valid variable name!"),
        };

        match &self.tokens[self.current] {
            Tokens::OperatorAssign => self.advance(),
            _ => panic!("Expected an '=' at variable declaration"),
        }

        ASTNode::VariableDecleration { name: var_name, var_type, value: Box::new() }
    } 
}