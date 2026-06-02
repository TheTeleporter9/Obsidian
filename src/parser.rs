use crate::AST::ASTNode;
use crate::token_table::{self, variable_table};
use crate::tokens::Tokens;

// ============================================================================
// DEVELOPMENT NOTE:
// Initial logic for this system was drafted using AI pseudocode.
// The entire codebase has since been manually rewritten, refactored, and
// engineered from scratch. Future development is entirely human-written.
// ============================================================================
pub struct Parser {
    pub tokens: Vec<Tokens>,
    position: usize,
    variable_table: token_table::variable_table,
}

impl Parser {
    pub fn new(input_tokens: Vec<Tokens>) -> Self {
        Self {
            tokens: input_tokens,
            position: 0,
            variable_table: token_table::variable_table::new(),
        }
    }

    //Helper functions
    fn peek() {}

    fn advance(&mut self) {
        if self.position <= self.tokens.len() {
            self.position += 1;
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut program_nodes = Vec::new();
        let mut variable_table: token_table::variable_table = variable_table::new();

        while self.position < self.tokens.len() {
            println!("{:?}", &self.tokens[self.position]);

            let node = match &self.tokens[self.position] {
                Tokens::VAR => self.parse_variable_declaration(),
                Tokens::PRINT => self.parse_print_statement(),
                Tokens::Identifier(_) => {
                    self.parse_expression()
                },

                _ => panic!("Unexpected TOken! {:?}", &self.tokens[self.position])
            };

            program_nodes.push(node);
        }

        program_nodes
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        self.advance(); // Skip VAR

        //Get vairable name
        let var_name = match &self.tokens[self.position] {
            Tokens::Identifier(name) => name.clone(),
            _ => panic!("Expected a valid variable name!"),
        };

        self.advance(); // Skip identifier

        match &self.tokens[self.position] {
            Tokens::OperatorAssign => self.advance(),
            _ => panic!("Expected an '=' at variable declaration"),
        }

        let value_node = self.parse_expression();

        token_table::variable_table::add_variable_reference(
            &mut self.variable_table,
            ASTNode::VariableDecleration {
                name: var_name.clone(),
                value: Box::new(value_node.clone()),
            },
        );

        ASTNode::VariableDecleration {
            name: var_name,
            value: Box::new(value_node),
        }
    }

    fn parse_print_statement(&mut self) -> ASTNode {
        self.advance();

        let target_node = self.parse_expression();

        return ASTNode::PrintDecleration {
            target: Box::new(target_node),
        };
    }

    fn parse_expression(&mut self) -> ASTNode {
        let left_node = match &self.tokens[self.position] {
            Tokens::LiteralInt(val) => {
                let value = *val;
                self.advance();
                ASTNode::LiteralInt { value }
            }
            Tokens::Identifier(name) => {
                if !self.variable_table.check_variable_reference(self.tokens[self.position].clone()) {
                    panic!("Variable '{}' is not defined!", name)
                }

            }
            _ => panic!("Expected number or variable"),
        };



        if self.position >= self.tokens.len() {
            return left_node;
        }

        //This is for LiteralInt type
        match self.tokens[self.position] {
            Tokens::OperatorAdd => {
                self.advance();
                let right_node = self.parse_expression();

                return ASTNode::BinaryOperaion {
                    left: Box::new(left_node),
                    operator: "+".to_string(),
                    right: Box::new(right_node),
                };
            }

            Tokens::OperatorSubtract => {
                self.advance();

                let right_node = self.parse_expression();

                return ASTNode::BinaryOperaion {
                    left: Box::new(left_node),
                    operator: "-".to_string(),
                    right: Box::new(right_node),
                };
            }

            Tokens::OperatorDivide => {
                self.advance();

                let right_node = self.parse_expression();

                return ASTNode::BinaryOperaion {
                    left: Box::new(left_node),
                    operator: "/".to_string(),
                    right: Box::new(right_node),
                };
            }

            Tokens::OperatorMultiply => {
                self.advance();

                let right_node = self.parse_expression();

                return ASTNode::BinaryOperaion {
                    left: Box::new(left_node),
                    operator: "*".to_string(),
                    right: Box::new(right_node),
                };
            }
            _ => {}
        }

        left_node
    }
}
