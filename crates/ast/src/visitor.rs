//! Паттерн посетитель для обхода AST

use crate::node::{ASTNode, Expr, Statement, Type, Parameter, BinaryOperator, UnaryOperator};

pub trait ASTVisitor {
    type Output;

    fn visit_program(&mut self, nodes: &[ASTNode]) -> Self::Output;
    fn visit_variable_declaration(&mut self, name: &str, data_type: &Type, initializer: &Option<Box<Expr>>) -> Self::Output;
    fn visit_function_declaration(&mut self, name: &str, parameters: &[Parameter], return_type: &Type, body: &[Statement]) -> Self::Output;
    fn visit_expression(&mut self, expr: &Expr) -> Self::Output;
    fn visit_statement(&mut self, stmt: &Statement) -> Self::Output;
    fn visit_binary_op(&mut self, left: &Expr, op: &BinaryOperator, right: &Expr) -> Self::Output;
    fn visit_unary_op(&mut self, op: &UnaryOperator, operand: &Expr) -> Self::Output;
}

pub struct PrintVisitor {
    indent_level: usize,
}

impl PrintVisitor {
    pub fn new() -> Self {
        PrintVisitor { indent_level: 0 }
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }
}

impl ASTVisitor for PrintVisitor {
    type Output = String;

    fn visit_program(&mut self, nodes: &[ASTNode]) -> Self::Output {
        let mut result = String::new();
        result.push_str("Program:\n");
        
        for node in nodes {
            result.push_str(&format!("{}{}\n", self.indent(), self.visit_node(node)));
        }
        
        result
    }

    fn visit_variable_declaration(&mut self, name: &str, data_type: &Type, initializer: &Option<Box<Expr>>) -> Self::Output {
        let type_str = match data_type {
            Type::Int => "int",
            Type::Char => "char",
            Type::Float => "float",
            Type::Double => "double",
            Type::Bool => "bool",
            Type::Pointer(inner) => &format!("ptr({})", self.visit_type(inner)),
            Type::Array(inner, size) => &format!("array({}, {})", self.visit_type(inner), size),
            Type::Custom(name) => name,
        };

        let init_str = match initializer {
            Some(expr) => format!(" = {}", self.visit_expression(expr)),
            None => String::new(),
        };

        format!("VariableDeclaration: {} {}{}", name, type_str, init_str)
    }

    fn visit_function_declaration(&mut self, name: &str, parameters: &[Parameter], return_type: &Type, body: &[Statement]) -> Self::Output {
        let params_str = parameters
            .iter()
            .map(|param| format!("{}: {}", param.name, self.visit_type(&param.param_type)))
            .collect::<Vec<_>>()
            .join(", ");

        let type_str = match return_type {
            Type::Int => "int",
            Type::Char => "char",
            Type::Float => "float",
            Type::Double => "double",
            Type::Bool => "bool",
            Type::Pointer(inner) => &format!("ptr({})", self.visit_type(inner)),
            Type::Array(inner, size) => &format!("array({}, {})", self.visit_type(inner), size),
            Type::Custom(name) => name,
        };

        format!("FunctionDeclaration: {}({}) -> {} {{ ... }}", name, params_str, type_str)
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        match expr {
            Expr::Identifier(name) => format!("Identifier({})", name),
            Expr::IntegerLiteral(value) => format!("IntegerLiteral({})", value),
            Expr::StringLiteral(value) => format!("StringLiteral({})", value),
            Expr::BinaryOp { left, operator, right } => {
                let op_str = match operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Sub => "-",
                    BinaryOperator::Mul => "*",
                    BinaryOperator::Div => "/",
                    BinaryOperator::Mod => "%",
                    BinaryOperator::Eq => "==",
                    BinaryOperator::Ne => "!=",
                    BinaryOperator::Lt => "<",
                    BinaryOperator::Le => "<=",
                    BinaryOperator::Gt => ">",
                    BinaryOperator::Ge => ">=",
                    BinaryOperator::And => "&&",
                    BinaryOperator::Or => "||",
                };
                
                format!("BinaryOp({} {} {})", 
                    self.visit_expression(left), 
                    op_str, 
                    self.visit_expression(right)
                )
            }
            Expr::UnaryOp { operator, operand } => {
                let op_str = match operator {
                    UnaryOperator::Neg => "-",
                    UnaryOperator::Not => "!",
                    UnaryOperator::AddrOf => "&",
                    UnaryOperator::Deref => "*",
                };
                
                format!("UnaryOp({}{})", op_str, self.visit_expression(operand))
            }
            Expr::FunctionCall { name, arguments } => {
                let args_str = arguments
                    .iter()
                    .map(|arg| self.visit_expression(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                    
                format!("FunctionCall({}({}))", name, args_str)
            }
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) -> Self::Output {
        match stmt {
            Statement::Return(expr) => {
                match expr {
                    Some(e) => format!("Return({})", self.visit_expression(e)),
                    None => "Return".to_string(),
                }
            }
            Statement::Expression(expr) => format!("ExpressionStmt({})", self.visit_expression(expr)),
            Statement::Block(stmts) => {
                let stmts_str = stmts
                    .iter()
                    .map(|stmt| format!("{}{}", self.indent(), self.visit_statement(stmt)))
                    .collect::<Vec<_>>()
                    .join("\n");
                    
                format!("Block {{\n{}{}\n{}}}", self.indent(), stmts_str, self.indent())
            }
            Statement::If { condition, then_branch, else_branch } => {
                let cond_str = self.visit_expression(condition);
                let then_str = then_branch
                    .iter()
                    .map(|stmt| format!("{}{}", self.indent(), self.visit_statement(stmt)))
                    .collect::<Vec<_>>()
                    .join("\n");
                    
                match else_branch {
                    Some(else_stmts) => {
                        let else_str = else_stmts
                            .iter()
                            .map(|stmt| format!("{}{}", self.indent(), self.visit_statement(stmt)))
                            .collect::<Vec<_>>()
                            .join("\n");
                            
                        format!("If({}) {{\n{}{}\n{}}} Else {{\n{}{}\n{}}}", 
                            cond_str, self.indent(), then_str, self.indent(), 
                            self.indent(), else_str, self.indent())
                    }
                    None => {
                        format!("If({}) {{\n{}{}\n{}}}", 
                            cond_str, self.indent(), then_str, self.indent())
                    }
                }
            }
            Statement::While { condition, body } => {
                let cond_str = self.visit_expression(condition);
                let body_str = body
                    .iter()
                    .map(|stmt| format!("{}{}", self.indent(), self.visit_statement(stmt)))
                    .collect::<Vec<_>>()
                    .join("\n");
                    
                format!("While({}) {{\n{}{}\n{}}}", 
                    cond_str, self.indent(), body_str, self.indent())
            }
        }
    }

    fn visit_binary_op(&mut self, left: &Expr, op: &BinaryOperator, right: &Expr) -> Self::Output {
        self.visit_expression(&Expr::BinaryOp {
            left: Box::new(left.clone()),
            operator: op.clone(),
            right: Box::new(right.clone()),
        })
    }

    fn visit_unary_op(&mut self, op: &UnaryOperator, operand: &Expr) -> Self::Output {
        self.visit_expression(&Expr::UnaryOp {
            operator: op.clone(),
            operand: Box::new(operand.clone()),
        })
    }
}

impl PrintVisitor {
    pub fn visit_node(&mut self, node: &ASTNode) -> String {
        match node {
            ASTNode::Program(nodes) => self.visit_program(nodes),
            ASTNode::VariableDeclaration { name, data_type, initializer } => {
                self.visit_variable_declaration(name, data_type, initializer)
            }
            ASTNode::FunctionDeclaration { name, parameters, return_type, body } => {
                self.visit_function_declaration(name, parameters, return_type, body)
            }
            ASTNode::Expression(expr) => self.visit_expression(expr),
            ASTNode::Statement(stmt) => self.visit_statement(stmt),
        }
    }

    fn visit_type(&mut self, ty: &Type) -> String {
        match ty {
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Pointer(inner) => format!("ptr({})", self.visit_type(inner)),
            Type::Array(inner, size) => format!("array({}, {})", self.visit_type(inner), size),
            Type::Custom(name) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{ASTNode, Expr, BinaryOperator};

    #[test]
    fn test_print_visitor() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        
        let node = ASTNode::Expression(expr);
        let mut visitor = PrintVisitor::new();
        let result = visitor.visit_node(&node);
        
        assert!(result.contains("BinaryOp"));
        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("+"));
    }
}