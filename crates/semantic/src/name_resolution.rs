//! Разрешение имён в C++ программе

use crate::symbol_table::SymbolTable;
use ast::{ASTNode, Expr, Statement};

#[derive(Debug, Clone, PartialEq)]
pub enum NameResolutionError {
    UndefinedVariable(String),
    VariableAlreadyDefined(String),
    FunctionAlreadyDefined(String),
    CannotAssignToConstant(String),
}

pub struct NameResolver {
    symbol_table: SymbolTable,
    errors: Vec<NameResolutionError>,
}

impl NameResolver {
    pub fn new() -> Self {
        NameResolver {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn resolve_program(&mut self, ast: &ASTNode) -> Result<(), Vec<NameResolutionError>> {
        match ast {
            ASTNode::Program(nodes) => {
                for node in nodes {
                    self.resolve_node(node);
                }
                
                if self.errors.is_empty() {
                    Ok(())
                } else {
                    Err(std::mem::take(&mut self.errors))
                }
            }
            _ => {
                self.errors.push(NameResolutionError::UndefinedVariable("Program node expected".to_string()));
                Err(std::mem::take(&mut self.errors))
            }
        }
    }

    fn resolve_node(&mut self, node: &ASTNode) -> bool {
        match node {
            ASTNode::VariableDeclaration { name, data_type: _, initializer } => {
                // Проверяем, не определена ли уже переменная с таким именем в текущей области
                if self.symbol_table.lookup_in_current_scope(name).is_some() {
                    self.errors.push(NameResolutionError::VariableAlreadyDefined(name.clone()));
                    return false;
                }
                
                // Добавляем переменную в таблицу символов
                let type_name = "unknown"; // В реальности здесь должен быть тип
                if let Err(_) = self.symbol_table.define_variable(name.clone(), type_name.to_string(), true) {
                    self.errors.push(NameResolutionError::VariableAlreadyDefined(name.clone()));
                    return false;
                }
                
                // Решаем инициализатор, если он есть
                if let Some(init_expr) = initializer {
                    self.resolve_expression(init_expr);
                }
                
                true
            }
            ASTNode::FunctionDeclaration { name, parameters, return_type: _, body } => {
                // Проверяем, не определена ли уже функция с таким именем
                if self.symbol_table.lookup(name).is_some() {
                    self.errors.push(NameResolutionError::FunctionAlreadyDefined(name.clone()));
                    return false;
                }
                
                // Добавляем параметры в таблицу символов
                self.symbol_table.enter_scope();
                
                for param in parameters {
                    if self.symbol_table.lookup_in_current_scope(&param.name).is_some() {
                        self.errors.push(NameResolutionError::VariableAlreadyDefined(param.name.clone()));
                        continue;
                    }
                    
                    let param_type = "unknown"; // В реальности здесь должен быть тип
                    if let Err(_) = self.symbol_table.define_variable(param.name.clone(), param_type.to_string(), true) {
                        self.errors.push(NameResolutionError::VariableAlreadyDefined(param.name.clone()));
                    }
                }
                
                // Решаем тело функции
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                
                self.symbol_table.exit_scope();
                
                true
            }
            ASTNode::Expression(expr) => {
                self.resolve_expression(expr);
                true
            }
            ASTNode::Statement(stmt) => {
                self.resolve_statement(stmt);
                true
            }
            ASTNode::Program(_) => {
                // Уже обработали в resolve_program
                true
            }
        }
    }

    fn resolve_statement(&mut self, stmt: &Statement) -> bool {
        match stmt {
            Statement::Return(expr) => {
                if let Some(return_expr) = expr {
                    self.resolve_expression(return_expr);
                }
                true
            }
            Statement::Expression(expr) => {
                self.resolve_expression(expr);
                true
            }
            Statement::Block(statements) => {
                self.symbol_table.enter_scope();
                
                for stmt in statements {
                    self.resolve_statement(stmt);
                }
                
                self.symbol_table.exit_scope();
                true
            }
            Statement::If { condition, then_branch, else_branch } => {
                // Решаем условие
                self.resolve_expression(condition);
                
                // Решаем ветвь then
                self.symbol_table.enter_scope();
                for stmt in then_branch {
                    self.resolve_statement(stmt);
                }
                self.symbol_table.exit_scope();
                
                // Решаем ветвь else, если есть
                if let Some(else_stmts) = else_branch {
                    self.symbol_table.enter_scope();
                    for stmt in else_stmts {
                        self.resolve_statement(stmt);
                    }
                    self.symbol_table.exit_scope();
                }
                
                true
            }
            Statement::While { condition, body } => {
                // Решаем условие
                self.resolve_expression(condition);
                
                // Решаем тело цикла
                self.symbol_table.enter_scope();
                for stmt in body {
                    self.resolve_statement(stmt);
                }
                self.symbol_table.exit_scope();
                
                true
            }
        }
    }

    fn resolve_expression(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(name) => {
                // Проверяем, определена ли переменная
                match self.symbol_table.lookup(name) {
                    Some(_) => true,
                    None => {
                        self.errors.push(NameResolutionError::UndefinedVariable(name.clone()));
                        false
                    }
                }
            }
            Expr::IntegerLiteral(_) | Expr::StringLiteral(_) => true,
            Expr::BinaryOp { left, right, .. } => {
                let left_ok = self.resolve_expression(left);
                let right_ok = self.resolve_expression(right);
                left_ok && right_ok
            }
            Expr::UnaryOp { operand, .. } => {
                self.resolve_expression(operand)
            }
            Expr::FunctionCall { name, arguments } => {
                // Проверяем, определена ли функция
                match self.symbol_table.lookup(name) {
                    Some(_) => {
                        // Решаем аргументы
                        for arg in arguments {
                            self.resolve_expression(arg);
                        }
                        true
                    }
                    None => {
                        self.errors.push(NameResolutionError::UndefinedVariable(name.clone()));
                        false
                    }
                }
            }
        }
    }

    pub fn get_errors(&self) -> &[NameResolutionError] {
        &self.errors
    }

    pub fn reset_errors(&mut self) {
        self.errors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Expr, ASTNode};

    #[test]
    fn test_name_resolver_basic() {
        let mut resolver = NameResolver::new();
        
        // Создаём объявление переменной: int x = 5;
        let decl = ASTNode::VariableDeclaration {
            name: "x".to_string(),
            data_type: ast::Type::Int,
            initializer: Some(Box::new(Expr::IntegerLiteral(5))),
        };
        
        let program = ASTNode::Program(vec![decl]);
        let result = resolver.resolve_program(&program);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_undefined_variable() {
        let mut resolver = NameResolver::new();
        
        // Создаём выражение: x + y (где y не определена)
        let expr = ASTNode::Expression(Expr::BinaryOp {
            left: Box::new(Expr::Identifier("x".to_string())),
            operator: ast::BinaryOperator::Add,
            right: Box::new(Expr::Identifier("y".to_string())),
        });
        
        // Сначала определяем x
        let x_decl = ASTNode::VariableDeclaration {
            name: "x".to_string(),
            data_type: ast::Type::Int,
            initializer: Some(Box::new(Expr::IntegerLiteral(5))),
        };
        
        let program = ASTNode::Program(vec![x_decl, expr]);
        let result = resolver.resolve_program(&program);
        
        // Ожидаем ошибку из-за неопределённой переменной y
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, NameResolutionError::UndefinedVariable(var) if var == "y")));
    }
}