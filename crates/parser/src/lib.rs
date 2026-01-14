//! Парсер для C++
//! Преобразует последовательность токенов в абстрактное синтаксическое дерево

mod grammar;
mod parser;
mod error;

pub use grammar::*;
pub use parser::Parser;
pub use error::ParseError;