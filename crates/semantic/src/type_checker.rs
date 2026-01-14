//! Проверка типов для C++

use crate::symbol_table::{SymbolTable, SymbolType};
use ast::node::{ASTNode, Expr, Statement, Type as AstType, BinaryOperator, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),
    TypeMismatch { expected: String, actual: String },
    InvalidOperation { operation: String, left: String, right: String },
    ReturnTypeMismatch { expected: String, actual: String },
    ArgumentCountMismatch { expected: usize, actual: usize },
    ArgumentTypeMismatch { index: usize, expected: String, actual: String },
}

pub struct TypeChecker {
    symbol_table: SymbolTable,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn check_program(&mut self, ast: &ASTNode) -> Result<(), TypeError> {
        match ast {
            ASTNode::Program(nodes) => {
                for node in nodes {
                    self.check_node(node)?;
                }
                Ok(())
            }
            _ => Err(TypeError::TypeMismatch { 
                expected: "Program".to_string(), 
                actual: "non-program node".to_string() 
            }),
        }
    }

    fn check_node(&mut self, node: &ASTNode) -> Result<(), TypeError> {
        match node {
            ASTNode::VariableDeclaration { name, data_type, initializer } => {
                // Добавляем переменную в таблицу символов
                let type_name = self.ast_type_to_string(data_type);
                self.symbol_table.define_variable(name.clone(), type_name.clone(), true)
                    .map_err(|_| TypeError::TypeMismatch { 
                        expected: "valid variable name".to_string(), 
                        actual: name.clone() 
                    })?;

                // Проверяем инициализатор, если он есть
                if let Some(init_expr) = initializer {
                    let init_type = self.infer_expression_type(init_expr)?;
                    if !self.types_compatible(&init_type, &type_name) {
                        return Err(TypeError::TypeMismatch {
                            expected: type_name,
                            actual: init_type,
                        });
                    }
                }
                
                Ok(())
            }
            ASTNode::FunctionDeclaration { name, parameters, return_type, body } => {
                // Добавляем функцию в таблицу символов
                let params: Vec<(String, String)> = parameters
                    .iter()
                    .map(|param| (param.name.clone(), self.ast_type_to_string(&param.param_type)))
                    .collect();
                
                let return_type_str = self.ast_type_to_string(return_type);
                
                let signature = crate::symbol_table::FunctionSignature {
                    return_type: return_type_str.clone(),
                    parameters: params,
                };
                
                self.symbol_table.define_function(name.clone(), signature)
                    .map_err(|_| TypeError::TypeMismatch { 
                        expected: "valid function name".to_string(), 
                        actual: name.clone() 
                    })?;
                
                // Временно добавляем параметры в таблицу символов
                self.symbol_table.enter_scope();
                for param in parameters {
                    let param_type = self.ast_type_to_string(&param.param_type);
                    self.symbol_table.define_variable(param.name.clone(), param_type, true)
                        .map_err(|_| TypeError::TypeMismatch { 
                            expected: "valid parameter name".to_string(), 
                            actual: param.name.clone() 
                        })?;
                }
                
                // Проверяем тело функции
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                
                // Проверяем, что все пути возвращают значение нужного типа (упрощённо)
                if return_type_str != "void" {
                    // Здесь должна быть проверка, что все пути возвращают значение
                }
                
                self.symbol_table.exit_scope();
                Ok(())
            }
            ASTNode::Expression(expr) => {
                self.infer_expression_type(expr)?;
                Ok(())
            }
            ASTNode::Statement(stmt) => {
                self.check_statement(stmt)?;
                Ok(())
            }
            ASTNode::Program(_) => {
                // Уже обработали в check_program
                Ok(())
            }
        }
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), TypeError> {
        match stmt {
            Statement::Return(expr) => {
                if let Some(return_expr) = expr {
                    let _expr_type = self.infer_expression_type(return_expr)?;
                    // Здесь должна быть проверка соответствия типа возвращаемого значения
                }
                Ok(())
            }
            Statement::Expression(expr) => {
                let _expr_type = self.infer_expression_type(expr)?;
                Ok(())
            }
            Statement::Block(statements) => {
                self.symbol_table.enter_scope();
                for stmt in statements {
                    self.check_statement(stmt)?;
                }
                self.symbol_table.exit_scope();
                Ok(())
            }
            Statement::If { condition, then_branch, else_branch } => {
                // Проверяем тип условия
                let cond_type = self.infer_expression_type(condition)?;
                if cond_type != "bool" && cond_type != "int" {  // Упрощение: int может использоваться как bool
                    return Err(TypeError::TypeMismatch {
                        expected: "bool".to_string(),
                        actual: cond_type,
                    });
                }
                
                // Проверяем ветвь then
                self.symbol_table.enter_scope();
                for stmt in then_branch {
                    self.check_statement(stmt)?;
                }
                self.symbol_table.exit_scope();
                
                // Проверяем ветвь else, если есть
                if let Some(else_stmts) = else_branch {
                    self.symbol_table.enter_scope();
                    for stmt in else_stmts {
                        self.check_statement(stmt)?;
                    }
                    self.symbol_table.exit_scope();
                }
                
                Ok(())
            }
            Statement::While { condition, body } => {
                // Проверяем тип условия
                let cond_type = self.infer_expression_type(condition)?;
                if cond_type != "bool" && cond_type != "int" {  // Упрощение
                    return Err(TypeError::TypeMismatch {
                        expected: "bool".to_string(),
                        actual: cond_type,
                    });
                }
                
                // Проверяем тело цикла
                self.symbol_table.enter_scope();
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.symbol_table.exit_scope();
                
                Ok(())
            }
        }
    }

    fn infer_expression_type(&mut self, expr: &Expr) -> Result<String, TypeError> {
        match expr {
            Expr::Identifier(name) => {
                match self.symbol_table.lookup(name) {
                    Some(symbol) => {
                        match &symbol.symbol_type {
                            SymbolType::Variable(type_name) => Ok(type_name.clone()),
                            SymbolType::Function(_) => Ok("function".to_string()),
                            SymbolType::TypeDefinition(_) => Ok("type".to_string()),
                        }
                    }
                    None => Err(TypeError::UndefinedVariable(name.clone())),
                }
            }
            Expr::IntegerLiteral(_) => Ok("int".to_string()),
            Expr::StringLiteral(_) => Ok("string".to_string()),
            Expr::BinaryOp { left, operator, right } => {
                let left_type = self.infer_expression_type(left)?;
                let right_type = self.infer_expression_type(right)?;
                
                // Проверяем совместимость типов для операции
                match operator {
                    BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div => {
                        if left_type != right_type {
                            return Err(TypeError::InvalidOperation {
                                operation: format!("{:?}", operator),
                                left: left_type,
                                right: right_type,
                            });
                        }
                        Ok(left_type) // Результат имеет тот же тип, что и операнды
                    }
                    BinaryOperator::Eq | BinaryOperator::Ne | BinaryOperator::Lt | 
                    BinaryOperator::Le | BinaryOperator::Gt | BinaryOperator::Ge => {
                        if left_type != right_type {
                            return Err(TypeError::InvalidOperation {
                                operation: format!("{:?}", operator),
                                left: left_type,
                                right: right_type,
                            });
                        }
                        Ok("bool".to_string()) // Операции сравнения возвращают bool
                    }
                    BinaryOperator::And | BinaryOperator::Or => {
                        if left_type != "bool" || right_type != "bool" {
                            return Err(TypeError::InvalidOperation {
                                operation: format!("{:?}", operator),
                                left: left_type,
                                right: right_type,
                            });
                        }
                        Ok("bool".to_string()) // Логические операции возвращают bool
                    }
                }
            }
            Expr::UnaryOp { operator, operand } => {
                let operand_type = self.infer_expression_type(operand)?;
                
                match operator {
                    UnaryOperator::Neg => {
                        if operand_type != "int" && operand_type != "float" && operand_type != "double" {
                            return Err(TypeError::InvalidOperation {
                                operation: format!("{:?}", operator),
                                left: operand_type,
                                right: "numeric type".to_string(),
                            });
                        }
                        Ok(operand_type)
                    }
                    UnaryOperator::Not => {
                        if operand_type != "bool" {
                            return Err(TypeError::InvalidOperation {
                                operation: format!("{:?}", operator),
                                left: operand_type,
                                right: "bool".to_string(),
                            });
                        }
                        Ok("bool".to_string())
                    }
                    _ => Ok(operand_type), // Остальные операции возвращают тип операнда
                }
            }
            Expr::FunctionCall { name, arguments } => {
                match self.symbol_table.lookup(name) {
                    Some(symbol) => {
                        match &symbol.symbol_type {
                            SymbolType::Function(signature) => {
                                // Проверяем количество аргументов
                                if arguments.len() != signature.parameters.len() {
                                    return Err(TypeError::ArgumentCountMismatch {
                                        expected: signature.parameters.len(),
                                        actual: arguments.len(),
                                    });
                                }
                                
                                // Проверяем типы аргументов
                                for (i, (arg, (_, expected_type))) in arguments.iter().zip(&signature.parameters).enumerate() {
                                    let arg_type = self.infer_expression_type(arg)?;
                                    if arg_type != *expected_type {
                                        return Err(TypeError::ArgumentTypeMismatch {
                                            index: i,
                                            expected: expected_type.clone(),
                                            actual: arg_type,
                                        });
                                    }
                                }
                                
                                Ok(signature.return_type.clone())
                            }
                            _ => Err(TypeError::TypeMismatch {
                                expected: "function".to_string(),
                                actual: "non-function".to_string(),
                            }),
                        }
                    }
                    None => Err(TypeError::UndefinedVariable(name.clone())),
                }
            }
        }
    }

    fn ast_type_to_string(&self, ast_type: &AstType) -> String {
        match ast_type {
            AstType::Int => "int".to_string(),
            AstType::Char => "char".to_string(),
            AstType::Float => "float".to_string(),
            AstType::Double => "double".to_string(),
            AstType::Bool => "bool".to_string(),
            AstType::Pointer(inner) => format!("ptr({})", self.ast_type_to_string(inner)),
            AstType::Array(inner, size) => format!("array({}, {})", self.ast_type_to_string(inner), size),
            AstType::Custom(name) => name.clone(),
        }
    }

    fn types_compatible(&self, actual: &str, expected: &str) -> bool {
        // Упрощённая проверка совместимости типов
        actual == expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::node::{ASTNode, Expr, BinaryOperator};

    #[test]
    fn test_type_checker_basic() {
        let mut checker = TypeChecker::new();
        
        // Создаём простое выражение: x + y
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("x".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Identifier("y".to_string())),
        };
        
        // Добавляем переменные в таблицу символов
        checker.symbol_table.define_variable("x".to_string(), "int".to_string(), true).unwrap();
        checker.symbol_table.define_variable("y".to_string(), "int".to_string(), true).unwrap();
        
        // Проверяем выражение
        let result = checker.infer_expression_type(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "int");
    }

    #[test]
    fn test_undefined_variable() {
        let mut checker = TypeChecker::new();
        
        let expr = Expr::Identifier("undefined_var".to_string());
        let result = checker.infer_expression_type(&expr);
        
        assert!(matches!(result, Err(TypeError::UndefinedVariable(_))));
    }
}