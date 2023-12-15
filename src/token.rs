use std::fmt::{Display, Formatter, Result};

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Token {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.literal {
            Some(lit) => write!(f, "{:?} {} {}", self.ttype, self.lexeme, lit),
            None => write!(f, "{:?} {} None", self.ttype, self.lexeme),
        }
    }
}
