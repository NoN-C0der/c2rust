//! Абстрактное синтаксическое дерево (AST) для C++
//! Представление программы в виде иерархии узлов

mod node;
mod visitor;
mod serializer;

pub use node::*;
pub use visitor::AstVisitor;
pub use serializer::AstSerializer;