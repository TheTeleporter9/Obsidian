use crate::token::Token;
use crate::ast::{ASTArena, ASTNode};

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
            panic!("Syntax Error: Expected token layout match error. Found: {:?}", current);
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
            Token::Var   => self.parse_variable_declaration(true),
            _            => self.parse_expression(),
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
    fn parse_expression(&mut self) -> usize {
        // Parse math precedence layers first
        let mut left_id = self.parse_math_addition();

        // Check if the resulting value is being piped downstream into a function
        while self.peek() == Some(&Token::SparkFeed) {
            self.advance(); // Skip the '-:' token

            // Grab the name of the function receiving the data
            let func_name = match self.advance() {
                Some(Token::Identifier(name)) => name,
                _ => panic!("Syntax Error: Expected target function identifier for pipeline feed"),
            };

            self.expect(Token::LParen); // Skip opening '('
            self.expect(Token::RParen); // Skip closing ')'

            // Create a pipeline step node linking back to the previous left_id index
            let pipeline_node = ASTNode::SparkFeedPipeline {
                input: left_id,
                function_name: func_name,
            };

            // Re-assign left_id to the pipeline index for any multi-stage piping
            left_id = self.arena.alloc(pipeline_node);
        }

        left_id
    }

    // Processes standard addition and subtraction binary math expression sequences
    fn parse_math_addition(&mut self) -> usize {
        let mut left_id = self.parse_primary();

        while self.peek() == Some(&Token::Plus) || self.peek() == Some(&Token::Minus) {
            let operator_token = self.advance().unwrap(); // Save '+' or '-' operator

            // Parse the right-hand value operand
            let right_id = self.parse_primary();

            // Construct our binary operator data block structure
            let math_node = ASTNode::BinaryOp {
                operator: operator_token,
                left: left_id,
                right: right_id,
            };

            // Allocate the math operation and update our running left index address
            left_id = self.arena.alloc(math_node);
        }

        left_id
    }

    // Resolves individual terminal literal values and basic variable names
    fn parse_primary(&mut self) -> usize {
        match self.advance() {
            Some(Token::IntLiteral(val))    => self.arena.alloc(ASTNode::IntLiteral(val)),
            Some(Token::FloatLiteral(val))  => self.arena.alloc(ASTNode::FloatLiteral(val)),
            Some(Token::BoolLiteral(val))   => self.arena.alloc(ASTNode::BoolLiteral(val)),
            Some(Token::StringLiteral(val)) => self.arena.alloc(ASTNode::StringLiteral(val)),
            Some(Token::Identifier(name))   => self.arena.alloc(ASTNode::Identifier(name)),
            Some(unrecognized) => {
                panic!("Syntax Error: Expected a core value. Found: {:?}", unrecognized);
            }
            None => panic!("Syntax Error: Encountered unexpected end of file bounds"),
        }
    }


}