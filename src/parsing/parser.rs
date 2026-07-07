use crate::tokens::Tokens;

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

    

    fn consume(&mut self, token: Tokens, error_message : String){
        if self.check(token) {
            self.advance();
        } else {
            panic!("{}",error_message)
        }
    }


}