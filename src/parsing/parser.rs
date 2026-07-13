use crate::DataType;
use crate::tokens::BinaryOperator;
use crate::tokens::ComparisonOperator;
use crate::tokens::LogicalOperator;
use crate::tokens::Tokens::OperatorAssign;
use crate::tokens::UnaryOperator;
use crate::{AST::ASTNode, tokens::Tokens};
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Tokens>,
    current: usize,
}

//Parser only converts tokens into AST, no typechecking and so on!
impl Parser {
    pub fn new(input_tokens: Vec<Tokens>) -> Self {
        Self {
            tokens: input_tokens,
            current: 0,
        }
    }

    pub fn parse_program(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            let node = match self.peek() {
                Tokens::VAR => self.parse_variable_declaration(),

                Tokens::PRINT => self.parse_print_statement(),

                Tokens::Identifier(_) => {
                    if self.peek_next() == Some(&Tokens::OperatorAssign) {
                        self.parse_assignment()
                    } else {
                        self.parse_expression()
                    }
                }

                Tokens::LiteralInt(_)
                | Tokens::OptionTrue
                | Tokens::OptionFalse
                | Tokens::BraceOpen
                | Tokens::OperatorSubtract
                | Tokens::OperatorAdd
                | Tokens::UnaryOperatorNot => self.parse_expression(),

                _ => panic!("Unknown Token!: {:?}", self.peek()),
            };

            nodes.push(node);
        }
        nodes
    }

    //helper functions

    fn peek(&self) -> &Tokens {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> Option<&Tokens> {
        self.tokens.get(self.current + 1)
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
        self.current >= self.tokens.len() || matches!(self.tokens[self.current], Tokens::EOF)
    }

    fn check(&self, token: Tokens) -> bool {
        !self.is_at_end() && self.peek() == &token
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
        self.parse_comparison()
    }

    fn parse_and(&mut self) -> ASTNode {
        let mut left = self.parse_comparison();

        while self.check(Tokens::OperatorAnd) {
            let operator = LogicalOperator::try_from(&self.tokens[self.current]).unwrap();
        }
    }

    fn parse_comparison(&mut self) -> ASTNode {
        let mut left = self.parse_additive();

        while self.check(Tokens::OperatorEqual)
            || self.check(Tokens::OperatorNotEqual)
            || self.check(Tokens::OperatorLess)
            || self.check(Tokens::OperatorLessEqual)
            || self.check(Tokens::OperatorGreater)
            || self.check(Tokens::OperatorGreaterEqual)
        {
            let operator = ComparisonOperator::try_from(&self.tokens[self.current]).unwrap();

            self.advance();

            let right = self.parse_additive();

            left = ASTNode::ComparisonOperator {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        left
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
        let mut left = self.parse_unary();

        while self.check(Tokens::OperatorMultiply) || self.check(Tokens::OperatorDivide) {
            let operator = BinaryOperator::try_from(&self.tokens[self.current]).unwrap();

            self.advance();

            let right = self.parse_unary();

            left = ASTNode::BinaryOperation {
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            }
        }

        left
    }

    fn parse_unary(&mut self) -> ASTNode {
        //If the current token matches
        if self.check(Tokens::UnaryOperatorNot)
            || self.check(Tokens::OperatorSubtract)
            || self.check(Tokens::OperatorAdd)
        {
            let operator = UnaryOperator::try_from(&self.tokens[self.current]).unwrap();

            self.advance();

            let operand = self.parse_unary();

            return ASTNode::UnaryOerator {
                operator: operator,
                operand: Box::new(operand),
            };
        }
        self.parse_primary()
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
                if self.peek_next() == Some(&Tokens::SquareBracketOpen) {
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

    fn parse_function_call(&mut self) -> ASTNode {
        let function_name = match &self.tokens[self.current] {
            Tokens::Identifier(name) => name.clone(),
            _ => panic!("Expected function name"),
        };

        self.advance(); // consume function name

        match &self.tokens[self.current] {
            Tokens::SquareBracketOpen => self.advance(),
            _ => panic!("Expected '['"),
        }

        let mut arguments = Vec::new();

        while !matches!(self.tokens[self.current], Tokens::SquareBracketClose) {
            arguments.push(self.parse_expression());

            if matches!(self.tokens[self.current], Tokens::Comma) {
                self.advance();
            }
        }

        self.advance(); // consume ']'

        ASTNode::FunctionCall {
            name: function_name,
            arguments,
        }
    }

    fn parse_print_statement(&mut self) -> ASTNode {
        self.advance();

        let target_node = self.parse_expression();

        return ASTNode::PrintDecleration {
            target: Box::new(target_node),
        };
    }

    fn parse_assignment(&mut self) -> ASTNode {
        let assingment_name = match &self.tokens[self.current] {
            Tokens::Identifier(name) => name.clone(),
            _ => panic!("Expected a valid variable name!"),
        };

        self.advance(); //consume the idintifier
        self.consume(OperatorAssign, "Expected '=' after identifier");
        //
        let value = self.parse_expression();

        ASTNode::Assignment {
            name: assingment_name,
            value: Box::new(value),
        }
    }
}
