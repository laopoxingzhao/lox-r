use std::{cell::RefCell, collections::HashMap, iter::Map, rc::Rc, sync::OnceLock, u32};

use crate::{
    err,
    token::{LiteralType, Token, TokenType},
};

pub static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

// 第一次使用时初始化
fn get_keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS.get_or_init(|| {
        HashMap::from([
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("for", TokenType::FOR),
            ("fun", TokenType::FUN),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
            // ("break", TokenType::BREAK),
            // ("continue", TokenType::CONTINUE),
            // ("const", TokenType::CONST),
            // ("export", TokenType::EXPORT),
        ])
    })
}
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        // 初始化静态变量
        get_keywords();
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        // 返回扫描到的所有token
        return self.tokens.clone();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_no_literal(TokenType::LEFT_PAREN),
            ')' => self.add_token_no_literal(TokenType::RIGHT_PAREN),
            '{' => self.add_token_no_literal(TokenType::LEFT_BRACE),
            '}' => self.add_token_no_literal(TokenType::RIGHT_BRACE),
            ',' => self.add_token_no_literal(TokenType::COMMA),
            '.' => self.add_token_no_literal(TokenType::DOT),
            '-' => self.add_token_no_literal(TokenType::MINUS),
            '+' => self.add_token_no_literal(TokenType::PLUS),
            ';' => self.add_token_no_literal(TokenType::SEMICOLON),
            '*' => self.add_token_no_literal(TokenType::STAR),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token_no_literal(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token_no_literal(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token_no_literal(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token_no_literal(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    // 处理单行注释
                    while !self.is_at_end() && self.advance() != '\n' {}
                } else if self.match_char('*') {
                    // 处理多行注释
                    while !self.is_at_end() {
                        if self.advance() == '*' && self.peek() == '/' {
                            self.advance(); // 跳过 '/'
                            break;
                        }
                    }
                } else {
                    self.add_token_no_literal(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            '0'..='9' => {
                self.number();
            }
            _ => {
                if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    err(self.line, &format!("Unexpected character: '{}'", c));
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current as usize).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            '\0'
        } else {
            self.source
                .chars()
                .nth((self.current + 1) as usize)
                .unwrap()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if !self.is_at_end() && self.source.chars().nth(self.current as usize) == Some(expected) {
            self.current += 1;
            return true;
        }
        false
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralType>) {
        let text = &self.source[self.start as usize..self.current as usize];
        // self.source.substring(self.start as usize, self.current as usize);
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }
    fn add_token_no_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            err(self.line, "Unterminated string.");
            return;
        }
        self.advance(); // 跳过结束的引号

        self.add_token(
            TokenType::STRING,
            Some(LiteralType::String(
                self.source[self.start as usize + 1..self.current as usize - 1].to_string(),
            )),
        );
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
        // 检查是否是小数
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance(); // 跳过小数点
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token(
            TokenType::NUMBER,
            Some(LiteralType::Number(
                self.source[self.start as usize..self.current as usize]
                    .parse()
                    .unwrap_or(0.0),
            )),
        );
    }
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start as usize..self.current as usize];
        if let Some(key) = get_keywords().get(text) {
            self.add_token_no_literal(*key);
            return;
        }

        self.add_token_no_literal(TokenType::IDENTIFIER);
    }
}
