//! Реализация лексера

use crate::{Token, LexerError};

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            let ch = self.peek_char();
            
            // Простая реализация для демонстрации
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    tokens.push(Token::new_identifier(ident));
                },
                '0'..='9' => {
                    let num = self.read_number();
                    tokens.push(Token::Number(num));
                },
                _ => {
                    self.position += ch.len_utf8();
                }
            }
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn peek_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.peek_char();
            if ch.is_alphanumeric() || ch == '_' {
                self.position += ch.len_utf8();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.peek_char();
            if ch.is_ascii_digit() {
                self.position += ch.len_utf8();
            } else {
                break;
            }
        }
        self.input[start..self.position].parse().unwrap_or(0)
    }
}