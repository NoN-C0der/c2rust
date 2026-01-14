//! Ошибки парсера

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEndOfInput,
    MismatchedBraces,
    ExpectedToken(String),
    InvalidExpression,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::MismatchedBraces => write!(f, "Mismatched braces"),
            ParseError::ExpectedToken(expected) => write!(f, "Expected token: {}", expected),
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::UnexpectedToken("int".to_string());
        assert_eq!(format!("{}", err), "Unexpected token: int");
    }
}