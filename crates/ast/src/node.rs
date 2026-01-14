//! Узлы абстрактного синтаксического дерева

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    /// Программа (корень AST)
    Program(Vec<ASTNode>),
    /// Объявление переменной
    VariableDeclaration {
        name: String,
        data_type: Type,
        initializer: Option<Box<Expr>>,
    },
    /// Объявление функции
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Statement>,
    },
    /// Выражение
    Expression(Expr),
    /// Оператор
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Float,
    Double,
    Bool,
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Идентификатор (переменная)
    Identifier(String),
    /// Целочисленная константа
    IntegerLiteral(i64),
    /// Строковая константа
    StringLiteral(String),
    /// Бинарное выражение (a + b, x * y и т.д.)
    BinaryOp {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    /// Унарное выражение (-x, !x и т.д.)
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    /// Вызов функции
    FunctionCall {
        name: String,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Возвращение значения
    Return(Option<Expr>),
    /// Выражение как оператор
    Expression(Expr),
    /// Блок кода
    Block(Vec<Statement>),
    /// Условный оператор
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    /// Цикл while
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Neg,
    Not,
    AddrOf,
    Deref,
}

impl ASTNode {
    pub fn new_variable_declaration(name: String, data_type: Type, initializer: Option<Expr>) -> Self {
        ASTNode::VariableDeclaration {
            name,
            data_type,
            initializer: initializer.map(Box::new),
        }
    }

    pub fn new_function_declaration(
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Statement>,
    ) -> Self {
        ASTNode::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_node_creation() {
        let var_decl = ASTNode::new_variable_declaration(
            "x".to_string(),
            Type::Int,
            Some(Expr::IntegerLiteral(42)),
        );
        
        assert!(matches!(var_decl, ASTNode::VariableDeclaration { .. }));
    }

    #[test]
    fn test_expr_creation() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        
        match expr {
            Expr::BinaryOp { left, operator, right } => {
                assert!(matches!(*left, Expr::Identifier(ref name) if name == "a"));
                assert_eq!(operator, BinaryOperator::Add);
                assert!(matches!(*right, Expr::Identifier(ref name) if name == "b"));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }
}