use std::{fmt::Display, u32};


#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}

#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<Literal>,
  pub line: u32,
}

impl Token {
  pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Self {
    Self { token_type, lexeme, literal, line }
  }
  pub fn to_string(&self) -> String {
    format!("{:?} {} {:#?}", self.token_type, self.lexeme, self.literal)
  }

}
#[derive(Debug,Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Literal{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}