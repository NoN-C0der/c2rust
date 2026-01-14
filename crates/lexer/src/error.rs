//! Ошибки лексера

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    /// Неподдерживаемый символ
    InvalidCharacter { 
        character: char, 
        position: usize 
    },
    /// Незавершенная строка
    UnterminatedString { 
        position: usize 
    },
    /// Незавершенный комментарий
    UnterminatedComment { 
        position: usize 
    },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidCharacter { character, position } => {
                write!(f, "Invalid character '{}' at position {}", character, position)
            },
            LexerError::UnterminatedString { position } => {
                write!(f, "Unterminated string starting at position {}", position)
            },
            LexerError::UnterminatedComment { position } => {
                write!(f, "Unterminated comment starting at position {}", position)
            },
        }
    }
}

impl std::error::Error for LexerError {}