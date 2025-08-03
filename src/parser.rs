use crate::expr::{Binary, Expr, Unary};
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
    fn expression(&mut self) -> Expr {
        self.equality()
    }
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let opr: Token = self.previous();
            let right = self.comparison();
            let b = Binary {
                left: Box::new(expr),
                operator: opr,
                right: Box::new(right),
            };
            expr = Expr::Binary(b);
        }
        return expr;
    }
    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
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
    fn previous(&self) -> Token {
        if self.current == 0 {
            return Token::new(TokenType::EOF, "".to_string(), None, 0);
        }
        self.tokens[self.current - 1].clone()
    }

    fn comparison(&mut self) -> Expr {
        let expr: Expr = self.term();

        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let opr: Token = self.previous();
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: opr,
                right: Box::new(right),
            })
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();
        while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
            let opr: Token = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: opr,
                right: Box::new(right),
            });
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();
        while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
            let opr: Token = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: opr,
                right: Box::new(right),
            });
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        /*   match self.peek().token_type {
        TokenType::FALSE => Expr::Literal(crate::expr::Literal {
            value: LiteralType::Bool(false),
        }),
        TokenType::TRUE => Expr::Literal(crate::expr::Literal {
            value: LiteralType::Bool(true),
        }),
        TokenType::NIL => Expr::Literal(crate::expr::Literal {
            value: LiteralType::Nil,
        }),
        TokenType::NUMBER | TokenType::STRING => {
            let literal = self.advance();
            Expr::Literal(crate::expr::Literal {
                value: self.previous().literal.clone().unwrap_or(LiteralType::Nil),
            })
        }
        TokenType::LEFT_PAREN => {
            self.advance(); // consume '('
            let expr = self.expression();
            if !self.match_token(&[TokenType::RIGHT_PAREN]) {
                panic!("Expected ')' after expression");
            }
            Expr::Grouping(crate::expr::Grouping {
                expr: Box::new(expr),
            })
        } */
        if self.match_token(&[TokenType::FALSE]) {
            let literal = self.previous();
        }
        if self.match_token(&[TokenType::TRUE]) {
            let literal = self.previous();
            return Expr::Literal(crate::expr::Literal {
                value: LiteralType::Bool(true),
            });
        }
        if self.match_token(&[TokenType::NIL]) {
            let literal = self.previous();
            return Expr::Literal(crate::expr::Literal {
                value: LiteralType::Nil,
            });
        }
        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            let literal = self.previous();
            return Expr::Literal(crate::expr::Literal {
                value: literal.literal.clone().unwrap_or(LiteralType::Nil),
            });
        }

        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            if !self.match_token(&[TokenType::RIGHT_PAREN]) {
                panic!("Expected ')' after expression");
            }
            return Expr::Grouping(crate::expr::Grouping {
                expr: Box::new(expr),
            });
        }
    //   return Expr::Literal(crate::expr::Literal { value: LiteralType::Nil });
    todo!("Primary expression parsing not implemented yet");
    }
}
