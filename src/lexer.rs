use crate::tokens::Tokens;
pub struct Lexer {
    source: Vec<char>,
    current_index_pos: usize,
    current_line: usize,
    current_colum: usize,
    pub tokens_out: Vec<Tokens>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            source: input.chars().collect(),
            current_index_pos: 0,
            current_line: 1,
            current_colum: 1,
            tokens_out: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) {
        let mut is_comment: bool = false;

        println!("STARING TONENIZATION!");

        while self.current_index_pos < self.source.len() {
            let current_char = self.source[self.current_index_pos];

            if current_char == '\n' {
                self.current_line += 1;
                self.current_colum = 1;
            } else {
                self.current_colum += 1
            }

            //Step1: skip white spaces and comments
            if current_char.is_whitespace() {
                self.current_index_pos += 1;
                continue;
            }

            //Make shure that the comment also gets closed, otherwise the entire code becomes a comment
            if current_char == '#' {
                is_comment = !is_comment;
                self.current_index_pos += 1;
                continue;
            }

            if is_comment {
                self.current_index_pos += 1;
                continue;
            }

            // Step2: Numbers
            if current_char.is_ascii_digit() {
                let mut literal_buffer_int = String::new();

                while self.current_index_pos < self.source.len()
                    && self.source[self.current_index_pos].is_ascii_digit()
                {
                    literal_buffer_int.push(self.source[self.current_index_pos]);

                    self.current_index_pos += 1;
                }

                self.tokens_out
                    .push(Tokens::LiteralInt(literal_buffer_int.parse().unwrap()));

                continue;
            }
            if current_char.is_ascii_alphabetic() || current_char == '_' {
                let mut literal_buffer_str = String::new();

                while self.current_index_pos < self.source.len()
                    && (self.source[self.current_index_pos].is_ascii_alphanumeric()
                        || self.source[self.current_index_pos] == '_')
                {
                    literal_buffer_str.push(self.source[self.current_index_pos]);
                    self.current_index_pos += 1;
                }

                self.tokens_out
                    .push(self.check_if_keyword(literal_buffer_str));

                continue;
            }
            // check if it is a spark (:-)
            if self.current_index_pos + 1 < self.source.len()
                && self.source[self.current_index_pos] == ':'
                && self.source[self.current_index_pos + 1] == '-'
            {
                self.tokens_out.push(Tokens::OperatorSpark);
                self.current_index_pos += 2;
                continue;
            }

            //Check if token matches a symbol
            if let Some(token) = self.check_for_symbol(current_char) {
                self.tokens_out.push(token);
                continue;
            }
            panic!(
                "Unexpected character: '{}', at line {:?}, colum {:?}",
                current_char, self.current_line, self.current_colum
            );
        }

        println!("TONENIZATION FINISHED!");
    }

    fn check_if_keyword(&self, string: String) -> Tokens {
        match string.as_str() {
            "const" => Tokens::CONST,
            "var" => Tokens::VAR,
            "print" => Tokens::PRINT,
            "func" => Tokens::FUNC,
            "int" => Tokens::TypeInt,
            "float" => Tokens::TypeFloat,
            "bool" => Tokens::TypeBoolean,

            "true" => Tokens::OptionTrue,
            "false" => Tokens::OptionFalse,

            _ => Tokens::Identifier(string),
        }
    }

    fn check_for_symbol(&mut self, character: char) -> Option<Tokens> {
        let token = match character {
            '=' => Some(Tokens::OperatorAssign),
            '+' => Some(Tokens::OperatorAdd),
            '-' => Some(Tokens::OperatorSubtract),
            '*' => Some(Tokens::OperatorMultiply),
            '/' => Some(Tokens::OperatorDivide),
            '(' => Some(Tokens::BraceOpen),
            ')' => Some(Tokens::BraceClose),
            ':' => Some(Tokens::OperatorSet),
            '{' => Some(Tokens::BracketOpen),
            '}' => Some(Tokens::BracketClose),
            '[' => Some(Tokens::SquareBracketOpen),
            ']' => Some(Tokens::SquareBracketClose),
            ',' => Some(Tokens::Comma),
            '!' => Some(Tokens::UnaryOperatorNot),
            _ => None,
        };

        if token.is_some() {
            self.current_index_pos += 1;
        }
        return token;
    }
}
