//! Промежуточное представление (IR) для C++
//! Универсальное представление программы между этапами компиляции

mod instruction;
mod basic_block;
mod function;

pub use instruction::*;
pub use basic_block::BasicBlock;
pub use function::Function;