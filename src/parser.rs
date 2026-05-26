use crate::ast::{ASTArena, ASTNode};
use crate::token::Token;

// Struct tracking tokens, cursor position, and the flat memory warehouse
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub arena: ASTArena,
}

impl Parser {
    // Initializes the parser state with the lexer's token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            arena: ASTArena::new(),
        }
    }

    // Looks at the token under the current cursor index without advancing
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // Advances the cursor by one position and returns a copy of the passed token
    fn advance(&mut self) -> Option<Token> {
        let tok = self.peek().cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    // Asserts that the current token matches what is expected, then advances over it
    fn expect(&mut self, expected: Token) -> Token {
        let current = self.peek();
        if current == Some(&expected) {
            self.advance().unwrap()
        } else {
            panic!(
                "Syntax Error: Expected token layout match error. Found: {:?}",
                current
            );
        }
    }

    // Master execution loop that steps through the file until hitting EOF
    pub fn parse_program(&mut self) -> Vec<usize> {
        let mut root_statements = Vec::new();

        while self.peek() != Some(&Token::EOF) {
            let stmt_id = self.parse_statement();
            root_statements.push(stmt_id);
        }

        root_statements
    }

    // Evaluates high-level keywords and routes them to specific parsing rules
    fn parse_statement(&mut self) -> usize {
        let token = match self.peek() {
            Some(tok) => tok,
            None => return self.arena.alloc(ASTNode::IntLiteral(0)), // Fallback safety
        };

        match token {
            Token::Const => self.parse_variable_declaration(false),
            Token::Var => self.parse_variable_declaration(true),
            Token::Print => self.parse_print_statement(),
            Token::Fn => self.parse_function_declaration(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression(),
        }
    }

    // Handles immutable 'const' and mutable 'var' structural definitions
    fn parse_variable_declaration(&mut self, is_mutable: bool) -> usize {
        self.advance(); // Skip the Const or Var keyword token

        // Extract the name label of the target variable
        let name = match self.advance() {
            Some(Token::Identifier(s)) => s,
            _ => panic!("Syntax Error: Expected variable label name identifier"),
        };

        // Check for an optional type annotation sequence (e.g., ': int')
        if self.peek() == Some(&Token::Colon) {
            self.advance(); // Skip ':'
            match self.advance() {
                Some(Token::Identifier(_)) => {} // Save or parse type name if tracking type info
                _ => panic!("Syntax Error: Expected type name after colon"),
            }
        }

        self.expect(Token::Assign); // Skip '='

        // Parse right-hand side expression from the inside out
        let initializer_id = self.parse_expression();

        // Bundle elements together into a flat memory container allocation unit
        let var_node = ASTNode::VariableDecleration {
            name,
            is_mutable,
            initializer: initializer_id,
        };

        // Commit definition record into our warehouse and return its index
        self.arena.alloc(var_node)
    }

    // Evaluates expression layers and manages the Spark Feed pipeline operator (-:)
    pub fn parse_expression(&mut self) -> usize {
        // Math addition handles + and -, and calls multiplication internally
        self.parse_additive_expression()
    }

    // 2. Handle Addition and Subtraction (+, -)
    fn parse_additive_expression(&mut self) -> usize {
        let mut left_id = self.parse_multiplicative_expression();

        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let operator = self.advance().unwrap(); // Consume the operator
                    let right_id = self.parse_multiplicative_expression();

                    left_id = self.arena.alloc(ASTNode::BinaryOp {
                        operator,
                        left: left_id,
                        right: right_id,
                    });
                }
                _ => break,
            }
        }

        // After math is done, check if there is a spark-feed pipeline attached to it
        if self.peek() == Some(&Token::SparkFeed) {
            self.advance(); // Skip '-:'
            let function_name = match self.advance() {
                Some(Token::Identifier(name)) => name,
                _ => panic!("Syntax Error: Expected target function identifier after '-:'"),
            };

            // Handle optional empty parentheses ()
            if self.peek() == Some(&Token::LParen) {
                self.advance(); // consume '('
                self.expect(Token::RParen); // consume ')'
            }

            left_id = self.arena.alloc(ASTNode::SparkFeedPipeline {
                input: left_id,
                function_name,
            });
        }

        left_id
    }

    // 3. Handle Multiplication and Division (*, /)
    fn parse_multiplicative_expression(&mut self) -> usize {
        let mut left_id = self.parse_primary(); // Base value (numbers, identifiers)

        while let Some(token) = self.peek() {
            match token {
                Token::Star | Token::Slash => {
                    let operator = self.advance().unwrap(); // Consume * or /
                    let right_id = self.parse_primary();

                    left_id = self.arena.alloc(ASTNode::BinaryOp {
                        operator,
                        left: left_id,
                        right: right_id,
                    });
                }
                _ => break,
            }
        }

        left_id
    }

    // Resolves individual terminal literal values and basic variable names
    fn parse_primary(&mut self) -> usize {
        match self.advance() {
            Some(Token::IntLiteral(val)) => self.arena.alloc(ASTNode::IntLiteral(val)),
            Some(Token::FloatLiteral(val)) => self.arena.alloc(ASTNode::FloatLiteral(val)),
            Some(Token::BoolLiteral(val)) => self.arena.alloc(ASTNode::BoolLiteral(val)),
            Some(Token::StringLiteral(val)) => self.arena.alloc(ASTNode::StringLiteral(val)),
            Some(Token::Identifier(name)) => self.arena.alloc(ASTNode::Identifier(name)),
            Some(unrecognized) => {
                panic!(
                    "Syntax Error: Expected a core value. Found: {:?}",
                    unrecognized
                );
            }
            None => panic!("Syntax Error: Encountered unexpected end of file bounds"),
        }
    }

    fn parse_print_statement(&mut self) -> usize {
        self.advance(); //skip the print keyword

        let value_id = self.parse_expression();

        let print_node = ASTNode::PrintStatement { value: value_id };
        self.arena.alloc(print_node)
    }

    fn parse_function_declaration(&mut self) -> usize {
        self.advance(); // Consume 'fn'

        let name = match self.advance() {
            Some(Token::Identifier(s)) => s,
            _ => panic!("Syntax Error: Expected function name identifier"),
        };

        self.expect(Token::LParen);
        let mut params = Vec::new();

        // Parse parameters list: (input: int, scalar: float)
        while self.peek() != Some(&Token::RParen) {
            let param_name = match self.advance() {
                Some(Token::Identifier(s)) => s,
                _ => panic!("Expected parameter name"),
            };
            self.expect(Token::Colon);
            let param_type = match self.advance() {
                Some(Token::Identifier(s)) => s,
                _ => panic!("Expected parameter type"),
            };

            params.push((param_name, param_type));

            if self.peek() == Some(&Token::Comma) {
                self.advance(); // Skip comma separator
            }
        }
        self.expect(Token::RParen);
        self.expect(Token::Arrow); // Skip '->'

        let return_type = match self.advance() {
            Some(Token::Identifier(s)) => s,
            _ => panic!("Expected return type identifier"),
        };

        self.expect(Token::LBrace); // Skip opening '{'

        // Parse the inner block body statements
        let mut body = Vec::new();
        while self.peek() != Some(&Token::RBrace) && self.peek() != Some(&Token::EOF) {
            body.push(self.parse_statement());
        }
        self.expect(Token::RBrace); // Skip closing '}'

        let fn_node = ASTNode::FunctionDeclaration {
            name,
            params,
            return_type,
            body,
        };
        self.arena.alloc(fn_node)
    }

    fn parse_return_statement(&mut self) -> usize {
        self.advance(); // Consume 'return' keyword

        // Check if there is an expression following the return statement
        let value = if self.peek() != Some(&Token::RBrace) && self.peek() != Some(&Token::EOF) {
            Some(self.parse_expression())
        } else {
            None
        };

        let return_node = ASTNode::ReturnStatement { value };
        self.arena.alloc(return_node)
    }
}
