//! Реализация парсера

use crate::{ParseError, precedence::Precedence};

pub struct Parser {
    tokens: Vec<lexer::Token>,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        Self { tokens, current_pos: 0 }
    }

    pub fn parse(&mut self) -> Result<ast::Program, ParseError> {
        // Заглушка для демонстрации
        Ok(ast::Program { statements: vec![] })
    }

    fn advance(&mut self) -> Option<&lexer::Token> {
        if self.current_pos < self.tokens.len() {
            self.current_pos += 1;
            self.current_token()
        } else {
            None
        }
    }

    fn current_token(&self) -> Option<&lexer::Token> {
        self.tokens.get(self.current_pos)
    }

    fn expect_token(&mut self, expected: &lexer::Token) -> Result<(), ParseError> {
        match self.current_token() {
            Some(token) if token == expected => {
                self.advance();
                Ok(())
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", self.current_token()),
            }),
        }
    }
}