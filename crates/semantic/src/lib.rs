//! Семантический анализатор для C++
//! Проверяет корректность программы после построения AST

mod symbol_table;
mod type_checker;
mod name_resolution;

pub use symbol_table::SymbolTable;
pub use type_checker::TypeChecker;
pub use name_resolution::NameResolver;