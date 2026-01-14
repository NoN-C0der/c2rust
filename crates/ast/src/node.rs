//! Узлы AST

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// Объявление переменной
    VariableDeclaration(VariableDecl),
    /// Объявление функции
    FunctionDeclaration(FunctionDecl),
    /// Выражение
    ExpressionStmt(Expression),
    /// Возвращение значения
    ReturnStmt(ReturnStmt),
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub r#type: Option<Type>,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub enum Expression {
    /// Идентификатор
    Identifier(String),
    /// Числовой литерал
    NumberLiteral(i64),
    /// Бинарное выражение
    BinaryOp(Box<BinaryOpExpr>),
    /// Вызов функции
    FunctionCall(FunctionCallExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryOpExpr {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

#[derive(Debug, Clone)]
pub struct FunctionCallExpr {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Void,
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
}