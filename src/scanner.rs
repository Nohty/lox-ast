use std::collections::HashMap;

use crate::{
    error::LoxError,
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;

        while !self.is_at_end() {
            self.start = self.current;

            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report();
                    had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));

        match had_error {
            Some(e) => Err(e),
            None => Ok(&self.tokens),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.is_match('/') {
                    while let Some(c) = self.peek() {
                        if c != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            '0'..='9' => self.number(),
            _ if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier();
            }
            _ => return Err(LoxError::new(self.line, "Unexpected character".to_string())),
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let result = self.source.get(self.current).unwrap();
        self.current += 1;

        return *result;
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_literal(ttype, None)
    }

    fn add_token_literal(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line))
    }

    fn is_match(&mut self, expected: char) -> bool {
        if let Some(c) = self.source.get(self.current) {
            if *c == expected {
                self.current += 1;
                return true;
            }
        }

        return false;
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(c) = self.peek() {
            match c {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new(self.line, "Unterminated string".to_string()));
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, Some(Literal::Str(value)));

        Ok(())
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let number = value.parse::<f64>().unwrap();

        self.add_token_literal(TokenType::Number, Some(Literal::Num(number)));
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let ttype = Scanner::keyword(&text).unwrap_or(TokenType::Identifier);

        self.add_token(ttype)
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    fn is_digit(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_ascii_digit(),
            None => false,
        }
    }

    fn is_alpha_numeric(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_ascii_alphanumeric() || c == '_',
            None => false,
        }
    }
}
