//! Токены языка C++

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Идентификатор
    Identifier(String),
    /// Числовая константа
    Number(i64),
    /// Строковая константа
    String(String),
    /// Комментарий
    Comment(String),
    /// Пробелы и символы новой строки
    Whitespace(String),
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

    pub fn new_string(value: String) -> Self {
        Token::String(value)
    }
}