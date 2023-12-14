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
        while !self.is_at_end() {
            self.start = self.current;

            match self.scan_token() {
                Ok(_) => {}
                Err(e) => e.report(),
            }
        }

        self.tokens.push(Token::eof(self.line));

        Ok(&self.tokens)
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
            _ => return Err(LoxError::new(self.line, "Unexpected character".to_string())),
        }

        Ok(())
    }

    fn advance(&mut self) -> &char {
        let result = self.source.get(self.current).unwrap();
        self.current += 1;

        return result;
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_literal(ttype, None)
    }

    fn add_token_literal(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line))
    }
}
