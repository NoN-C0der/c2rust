//! Ошибки парсера

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Неожиданный токен
    UnexpectedToken {
        expected: String,
        found: String,
    },
    /// Ожидается выражение
    ExpectedExpression,
    /// Ожидается идентификатор
    ExpectedIdentifier,
    /// Ошибка при разборе выражения
    ExpressionParseError(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Unexpected token: expected {}, found {}", expected, found)
            },
            ParseError::ExpectedExpression => {
                write!(f, "Expected expression")
            },
            ParseError::ExpectedIdentifier => {
                write!(f, "Expected identifier")
            },
            ParseError::ExpressionParseError(msg) => {
                write!(f, "Expression parse error: {}", msg)
            },
        }
    }
}

impl std::error::Error for ParseError {}