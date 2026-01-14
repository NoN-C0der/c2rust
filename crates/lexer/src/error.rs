//! Ошибки лексера

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnterminatedString,
    InvalidEscapeSequence(char),
    IoError(String),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(ch) => write!(f, "Unexpected character: {}", ch),
            LexerError::UnterminatedString => write!(f, "Unterminated string literal"),
            LexerError::InvalidEscapeSequence(ch) => write!(f, "Invalid escape sequence: \\{}", ch),
            LexerError::IoError(msg) => write!(f, "IO Error: {}", msg),
        }
    }
}

impl std::error::Error for LexerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LexerError::UnexpectedCharacter('!');
        assert_eq!(format!("{}", err), "Unexpected character: !");
    }
}