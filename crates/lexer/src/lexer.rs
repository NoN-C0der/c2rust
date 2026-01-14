//! Лексер для разбора исходного кода C++

use std::iter::Peekable;
use std::str::Chars;

use crate::{Token, LexerError};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.peek().copied();
        
        Lexer {
            input: chars,
            current_char,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        // Пропускаем пробельные символы
        self.skip_whitespace();

        match self.current_char {
            Some(ch) => {
                if ch.is_alphabetic() || ch == '_' {
                    return Ok(self.read_identifier());
                } else if ch.is_numeric() {
                    return Ok(self.read_number());
                } else {
                    match ch {
                        '"' => Ok(self.read_string()),
                        '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '%' | '~' => {
                            // Операторы
                            let op = ch.to_string();
                            self.advance();
                            Ok(Token::Identifier(op))
                        }
                        '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' => {
                            // Разделители
                            let sep = ch.to_string();
                            self.advance();
                            Ok(Token::Identifier(sep))
                        }
                        _ => {
                            self.advance();
                            Err(LexerError::UnexpectedCharacter(ch))
                        }
                    }
                }
            }
            None => Ok(Token::Eof),
        }
    }

    fn advance(&mut self) {
        self.input.next();
        self.current_char = self.input.peek().copied();
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        Token::Identifier(ident)
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_numeric() {
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        if let Ok(value) = num.parse::<i64>() {
            Token::Number(value)
        } else {
            Token::Identifier(num)
        }
    }

    fn read_string(&mut self) -> Token {
        let mut s = String::new();
        self.advance(); // пропустить открывающую кавычку
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // пропустить закрывающую кавычку
                break;
            } else {
                s.push(ch);
                self.advance();
            }
        }
        
        Token::String(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "int x = 42;";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("int".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("=".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Number(42));
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier(";".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }
}