//! Инструкции IR

use types::Type;

#[derive(Debug, Clone)]
pub enum Instruction {
    /// Присваивание константы
    Const {
        dest: String,
        value: Constant,
    },
    /// Бинарная операция
    BinaryOp {
        dest: String,
        op: BinaryOperator,
        left: Operand,
        right: Operand,
    },
    /// Переход без условия
    Jump {
        target: String,
    },
    /// Условный переход
    Branch {
        condition: Operand,
        true_target: String,
        false_target: String,
    },
    /// Возврат значения
    Return {
        value: Option<Operand>,
    },
    /// Вызов функции
    Call {
        dest: Option<String>,
        function: String,
        args: Vec<Operand>,
    },
    /// Загрузка значения из памяти
    Load {
        dest: String,
        source: Operand,
    },
    /// Сохранение значения в память
    Store {
        dest: Operand,
        source: Operand,
    },
    /// Выделение памяти
    Alloca {
        dest: String,
        element_type: Type,
    },
}

#[derive(Debug, Clone)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Operand {
    /// Имя переменной
    Variable(String),
    /// Константа
    Constant(Constant),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

impl Instruction {
    pub fn new_const(dest: String, value: Constant) -> Self {
        Instruction::Const { dest, value }
    }

    pub fn new_binary_op(
        dest: String,
        op: BinaryOperator,
        left: Operand,
        right: Operand,
    ) -> Self {
        Instruction::BinaryOp {
            dest,
            op,
            left,
            right,
        }
    }

    pub fn new_return(value: Option<Operand>) -> Self {
        Instruction::Return { value }
    }
}