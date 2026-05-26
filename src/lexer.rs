use crate::token::Token;

// Holds the source character array and tracking cursor index
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    // Initializes the lexer state from a source string
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    // Looks at the current character without moving the cursor forward
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    // Looks at the next character ahead without moving the cursor forward
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    // Returns the current character and advances the cursor index by one
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    // Consumes alphanumeric characters and returns a Keyword or Identifier
    fn scan_identifier(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }

        let slice: String = self.chars[start..self.pos].iter().collect();

        // Map matching string slices directly to explicit keyword variants
        match slice.as_str() {
            "package" => Token::Package,
            "import" => Token::Import,
            "const" => Token::Const,
            "var" => Token::Var,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "class" => Token::Class,
            "impl" => Token::Impl,
            "interface" => Token::Interface,
            "enum" => Token::Enum,
            "self" => Token::SelfLower,
            "Self" => Token::SelfUpper,
            "pub" => Token::Pub,
            "priv" => Token::Priv,
            "mut" => Token::Mut,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "match" => Token::Match,
            "fail" => Token::Fail,
            "pass" => Token::Pass,
            "guard" => Token::Guard,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "defer" => Token::Defer,
            "alloc" => Token::Alloc,
            "free" => Token::Free,
            "async" => Token::Async,
            "await" => Token::Await,
            "spawn" => Token::Spawn,
            "scope" => Token::Scope,
            "comptime" => Token::Comptime,
            "true" => Token::BoolLiteral(true),
            "false" => Token::BoolLiteral(false),
            "none" => Token::None,
            "some" => Token::Some,
            "void" => Token::Void,
            "never" => Token::Never,
            "any" => Token::Any,
            "in" => Token::In,
            "or" => Token::Or,
            "step" => Token::Step,
            "println" => Token::Print,
            _ => Token::Identifier(slice),
        }
    }

    // Distinguishes between integer literals and decimal float literals
    fn scan_number(&mut self) -> Token {
        let start = self.pos;
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else if c == '.'
                && self.peek_next().map_or(false, |next| next.is_ascii_digit())
                && !is_float
            {
                // Ensure dot is followed by a digit to avoid consuming range operators
                is_float = true;
                self.pos += 1;
            } else {
                break;
            }
        }

        let slice: String = self.chars[start..self.pos].iter().collect();

        if is_float {
            slice
                .parse()
                .map(Token::FloatLiteral)
                .unwrap_or(Token::Unknown('.'))
        } else {
            slice
                .parse()
                .map(Token::IntLiteral)
                .unwrap_or(Token::Unknown('0'))
        }
    }

    // Consumes literal characters enclosed within double quotes
    fn scan_string(&mut self) -> Token {
        self.advance(); // Consume opening quote
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.pos += 1;
        }

        let value: String = self.chars[start..self.pos].iter().collect();
        self.advance(); // Consume closing quote

        Token::StringLiteral(value)
    }

    // Processes individual layout components and routes compound operators
    fn next_token(&mut self) -> Token {
        // Discards whitespace and skips single-line code comments
        loop {
            let c = match self.peek() {
                Some(ch) => ch,
                None => return Token::EOF,
            };

            if c.is_whitespace() {
                self.advance();
                continue;
            }

            if c == '/' && self.peek_next() == Some('/') {
                self.advance(); // Skip first slash
                self.advance(); // Skip second slash
                while let Some(comment_ch) = self.peek() {
                    self.advance();
                    if comment_ch == '\n' {
                        break;
                    }
                }
                continue;
            }

            break;
        }

        // Re-evaluate character bounds after discarding layout overhead
        let c = match self.peek() {
            Some(ch) => ch,
            None => return Token::EOF,
        };

        // Handle word identifiers and keyword symbols
        if c.is_ascii_alphabetic() || c == '_' {
            return self.scan_identifier();
        }

        // Handle numeric values
        if c.is_ascii_digit() {
            return self.scan_number();
        }

        // Handle punctuation, math operators, and multi-character operators
        match c {
            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Equal
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::FatArrow
                } else {
                    Token::Assign
                }
            }

            '-' => {
                self.advance();
                if self.peek() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else if self.peek() == Some(':') {
                    self.advance();
                    Token::SparkFeed
                } else {
                    Token::Minus
                }
            }

            ':' => {
                self.advance();
                if self.peek() == Some(':') {
                    self.advance();
                    Token::Namespace
                } else {
                    Token::Colon
                }
            }

            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else if self.peek() == Some('!') {
                    self.advance();
                    Token::Assert
                } else {
                    Token::Not
                }
            }

            '.' => {
                self.advance();
                if self.peek() == Some('.') {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::RangeIncl
                    } else {
                        Token::Range
                    }
                } else {
                    Token::Dot
                }
            }

            '?' => {
                self.advance();
                if self.peek() == Some('.') {
                    self.advance();
                    Token::OptionalChain
                } else {
                    Token::Unknown('?')
                }
            }

            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LessEq
                } else if self.peek() == Some('<') {
                    self.advance();
                    Token::Shl
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::Concat
                } else {
                    Token::Less
                }
            }

            '>' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GreaterEq
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::Shr
                } else {
                    Token::Greater
                }
            }

            '&' => {
                self.advance();
                if self.peek() == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    Token::BitAnd
                }
            }

            '|' => {
                self.advance();
                if self.peek() == Some('|') {
                    self.advance();
                    Token::OrOp
                } else {
                    Token::BitOr
                }
            }

            '*' => {
                self.advance();
                if self.peek() == Some('*') {
                    self.advance();
                    Token::Power
                } else {
                    Token::Star
                }
            }

            // Single character assignments and delimiters
            '+' => {
                self.advance();
                Token::Plus
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '%' => {
                self.advance();
                Token::Percent
            }
            '^' => {
                self.advance();
                Token::BitXor
            }
            '~' => {
                self.advance();
                Token::BitNot
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '{' => {
                self.advance();
                Token::LBrace
            }
            '}' => {
                self.advance();
                Token::RBrace
            }
            '[' => {
                self.advance();
                Token::LBracket
            }
            ']' => {
                self.advance();
                Token::RBracket
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            '"' => self.scan_string(),

            // Fallback path handles unrecognized single tokens safely
            unrecognized_char => {
                self.advance();
                Token::Unknown(unrecognized_char)
            }
        }
    }

    // Driver sequence loop running until EOF terminal boundary is encountered
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let tok = self.next_token();
            if tok == Token::EOF {
                tokens.push(Token::EOF);
                break;
            }
            tokens.push(tok);
        }

        tokens
    }
}
