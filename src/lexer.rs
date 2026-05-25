use crate::token::Token;

pub struct Lexer {
    pub source_characters: Vec<char>,
    pub cursor_position: usize,
}

impl Lexer {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens_list: Vec<Token> = vec![];

        while self.cursor_position < self.source_characters.len() {
            let current_char =
                self.source_characters[self.cursor_position];

            if current_char.is_whitespace() {
                self.advance_cursor(1);
                continue;
            }

            if current_char == '=' {
                tokens_list.push(Token::Assign);
                self.advance_cursor(1);
            }
            else if current_char == '-' {
                if self.cursor_position + 1 < self.source_characters.len()
                    && self.source_characters[self.cursor_position + 1] == ':' 
                {
                    tokens_list.push(Token::SparkFeed);
                    self.advance_cursor(2);
                } else if self.cursor_position + 1 < self.source_characters.len()
                    && self.source_characters[self.cursor_position + 1] == '>' 
                {
                    tokens_list.push(Token::Arrow);
                    self.advance_cursor(2);
                } else {
                    tokens_list.push(Token::Minus);
                    self.advance_cursor(1);
                }
            }
            else if current_char.is_ascii_digit() {
                let mut literalBuffer = String::new();

                while self.cursor_position < self.source_characters.len()
                    && self.source_characters[self.cursor_position].is_ascii_digit()
                {
                    literalBuffer.push(
                        self.source_characters[self.cursor_position]
                    );
                    self.advance_cursor(1);
                }

                tokens_list.push(
                    Token::IntLiteral(literalBuffer.parse().unwrap())
                );

                continue;
            }
           else if current_char.is_alphabetic() {
    let mut identifier_buffer = String::new();

    while self.cursor_position < self.source_characters.len()
        && self.source_characters[self.cursor_position].is_alphanumeric()
    {
        identifier_buffer.push(
            self.source_characters[self.cursor_position]
        );
        self.advance_cursor(1);
    }

    match identifier_buffer.as_str() {
        // =========================
        // Keywords
        // =========================
        "package" => tokens_list.push(Token::Package),
        "import"  => tokens_list.push(Token::Import),

        "const"   => tokens_list.push(Token::Const),
        "var"     => tokens_list.push(Token::Var),

        "fn"      => tokens_list.push(Token::Fn),
        "return"  => tokens_list.push(Token::Return),

        "class"   => tokens_list.push(Token::Class),
        "impl"    => tokens_list.push(Token::Impl),
        "interface" => tokens_list.push(Token::Interface),
        "enum"    => tokens_list.push(Token::Enum),

        "self"    => tokens_list.push(Token::SelfLower),
        "Self"    => tokens_list.push(Token::SelfUpper),

        "pub"     => tokens_list.push(Token::Pub),
        "priv"    => tokens_list.push(Token::Priv),
        "mut"     => tokens_list.push(Token::Mut),

        "if"      => tokens_list.push(Token::If),
        "else"    => tokens_list.push(Token::Else),
        "for"     => tokens_list.push(Token::For),
        "while"   => tokens_list.push(Token::While),
        "loop"    => tokens_list.push(Token::Loop),
        "break"   => tokens_list.push(Token::Break),
        "continue"=> tokens_list.push(Token::Continue),
        "match"   => tokens_list.push(Token::Match),

        "fail"    => tokens_list.push(Token::Fail),
        "pass"    => tokens_list.push(Token::Pass),
        "guard"   => tokens_list.push(Token::Guard),
        "try"     => tokens_list.push(Token::Try),
        "catch"   => tokens_list.push(Token::Catch),

        "defer"   => tokens_list.push(Token::Defer),
        "alloc"   => tokens_list.push(Token::Alloc),
        "free"    => tokens_list.push(Token::Free),

        "async"   => tokens_list.push(Token::Async),
        "await"   => tokens_list.push(Token::Await),
        "spawn"   => tokens_list.push(Token::Spawn),
        "scope"   => tokens_list.push(Token::Scope),

        "comptime"=> tokens_list.push(Token::Comptime),

        "true"    => tokens_list.push(Token::True),
        "false"   => tokens_list.push(Token::False),
        "none"    => tokens_list.push(Token::None),
        "some"    => tokens_list.push(Token::Some),

        "void"    => tokens_list.push(Token::Void),
        "never"   => tokens_list.push(Token::Never),
        "any"     => tokens_list.push(Token::Any),

        "in"      => tokens_list.push(Token::In),
        "or"      => tokens_list.push(Token::Or),
        "step"    => tokens_list.push(Token::Step),

        // =========================
        // Default = identifier
        // =========================            
        _ => tokens_list.push(Token::Identifier(identifier_buffer)),
    }

    continue;
} 
            self.advance_cursor(1);
        }
        tokens_list.push(Token::EOF);


        return tokens_list;
    }

    fn advance_cursor(&mut self, amount: i32) {
        self.cursor_position += amount as usize;
    }
}