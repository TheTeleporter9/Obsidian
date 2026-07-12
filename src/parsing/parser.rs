use std::fmt::Binary;

use crate::DataType;
use crate::tokens::BinaryOperator;
use crate::{
    AST::{self, ASTNode},
    tokens::Tokens::{self, Identifier},
};
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Tokens>,
    current: usize,
}

impl Parser {
    pub fn new(input_tokens: Vec<Tokens>) -> Self {
        Self {
            tokens: input_tokens,
            current: 0,
        }
    }

    pub fn parse_program(&mut self) {
        while !self.is_at_end() {
            match (self.tokens[self.current]) {
                Tokens::EOF => panic!("End of file reached! Yay please implement this!"),
                Tokens::VAR => self.advance(),
                _ => panic!("Unknown Token!"),
            }
        }
    }

    //helper functions

    fn peek(&self) -> &Tokens {
        if self.current + 1 < self.tokens.len() {
            return &self.tokens[self.current + 1];
        }
        return &self.tokens[self.current];
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn previous(&mut self) -> Tokens {
        if self.current > 0 {
            return self.tokens[self.current - 1].clone();
        } else {
            return self.tokens[self.current].clone();
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.tokens.len();
    }

    fn check(&self, token: Tokens) -> bool {
        self.tokens[self.current] == token
    }

    fn advance_if_token(&mut self, token: Tokens) {
        if token == self.tokens[self.current] {
            self.advance();
        }
    }

    fn consume(&mut self, token: Tokens, error_message: &str) {
        if self.check(token) {
            self.advance();
        } else {
            panic!("{}", error_message)
        }
    }

    fn get_current_token(&self) -> &Tokens {
        return &self.tokens[self.current];
    }
}

impl Parser {
    fn parse_expression(&mut self) -> ASTNode {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> ASTNode {
        let mut left: ASTNode = self.parse_multiplicative();

        while self.check(Tokens::OperatorAdd) || self.check(Tokens::OperatorSubtract) {
            let operator = BinaryOperator::try_from(&self.tokens[self.current]).unwrap();

            self.advance();

            let right = self.parse_multiplicative();

            left = ASTNode::BinaryOperation {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            }
        }
        left
    }

    fn parse_multiplicative(&mut self) -> ASTNode {
        let mut left = self.parse_primary();

        while self.check(Tokens::OperatorMultiply) || self.check(Tokens::OperatorDivide) {
            let operator = BinaryOperator::try_from(&self.tokens[self.current]).unwrap();

            self.advance();

            let right = self.parse_primary();

            left = ASTNode::BinaryOperation {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            }
        }

        left
    }

    fn parse_primary(&mut self) -> ASTNode {
        match &self.tokens[self.current] {
            Tokens::LiteralInt(value) => {
                let value = *value;
                self.advance();

                ASTNode::LiteralInt { value }
            }

            Tokens::OptionTrue => {
                self.advance();
                ASTNode::LiteralBool { value: true }
            }

            Tokens::OptionFalse => {
                self.advance();

                ASTNode::LiteralBool { value: false }
            }

            Tokens::Identifier(name) => {
                if self.peek() == &Tokens::SquareBracketOpen {
                    return self.parse_function_call();
                }

                let name = name.clone();
                self.advance();

                ASTNode::Identifier { name }
            }

            Tokens::BraceOpen => {
                self.advance(); //comsume '('

                let expression = self.parse_expression();

                self.consume(Tokens::BraceClose, "Expected ')' after expression");

                expression
            }

            _ => panic!("Expected expression, found {:?}", self.tokens[self.current]),
        }
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
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

        self.advance();

        //Consulme the = sign
        match &self.tokens[self.current] {
            Tokens::OperatorAssign => self.advance(),
            _ => panic!("Expected an '=' at variable declaration"),
        }

        let var_value = self.parse_expression();

        ASTNode::VariableDecleration {
            name: var_name,
            var_type,
            value: Box::new(var_value),
        }
    }

    fn parse_function_call(&mut self) -> ASTNode {}
}
