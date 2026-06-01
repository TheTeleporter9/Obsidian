use crate::AST::ASTNode;
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
}

impl Parser {
    pub fn new(input_tokens: Vec<Tokens>) -> Self {
        Self {
            tokens: input_tokens,
            position: 0,
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

        while self.position < self.tokens.len() {
            let node = match &self.tokens[self.position] {
                Tokens::VAR => self.parse_variable_declaration(),
                Tokens::PRINT => self.parse_print_statement(),

                _ => panic!("Unexpected Token!"),
            };

            program_nodes.push(node);
        }

        program_nodes
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        self.advance(); // Skip VAR

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
            Tokens::LiteralInt(int_value) => {
                let value = *int_value;
                self.advance();
                ASTNode::LiteralInt { value }
            }
            Tokens::Identifier(identi_value) => {
                let name = identi_value.clone();
                self.advance();
                ASTNode::Identifier { name }
            }
            _ => panic!("Expected integer or a variable"),
        };

        if self.position < self.tokens.len()
            && matches!(self.tokens[self.position], Tokens::OperatorAdd)
        {
            self.advance();

            let right_node = self.parse_expression();

            ASTNode::BinaryOperaion {
                left: Box::new(left_node),
                operator: "+".to_string(),
                right: Box::new(right_node),
            }
        } else {
            left_node
        }
    }
}
