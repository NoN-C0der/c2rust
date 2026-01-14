//! Парсер для C++
//! Преобразует последовательность токенов в абстрактное синтаксическое дерево

mod grammar;
mod parser;
mod error;

// Заглушка для ASTNode до тех пор, пока не будет создан модуль AST
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Declaration,
    ExpressionStatement,
    Block(Vec<ASTNode>),
}

pub use grammar::*;
pub use parser::Parser;
pub use error::ParseError;