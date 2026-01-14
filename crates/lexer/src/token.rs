//! Токены лексера

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Идентификатор
    Identifier(String),
    /// Числовой литерал
    Number(i64),
    /// Строковый литерал
    String(String),
    /// Комментарий
    Comment(String),
    /// Пробел
    Whitespace,
    /// Конец файла
    Eof,
}

impl Token {
    pub fn new_identifier(name: String) -> Self {
        Token::Identifier(name)
    }

    pub fn new_number(value: i64) -> Self {
        Token::Number(value)
    }
}