use crate::expr::Expr;
use crate::scanner::Scanner;
use crate::token::{LiteralType, Token, TokenType};

struct Parse {
    // scanner: Scanner,
    tokens: Vec<Token>,
    current: usize,
}
impl Parse {
    pub fn new(scanner: Scanner) -> Self {
        Self {
            // scanner,
            tokens: scanner.tokens,
            current: 0,
        }
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self)-> bool {
        // self.current >= self.tokens.len()
        self.peek().token_type == TokenType::EOF
    }
    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            return Token::new(TokenType::EOF, "".to_string(), None, 0);
        }
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }



    fn  unary(&mut self) -> Expr {
       
      
    }
    


}
