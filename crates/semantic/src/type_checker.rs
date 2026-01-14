//! Проверка типов

use crate::SymbolTable;
use ast::node::*;

pub struct TypeChecker {
    symbol_table: SymbolTable,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VariableDeclaration(var_decl) => self.check_variable_declaration(var_decl),
            Statement::FunctionDeclaration(func_decl) => self.check_function_declaration(func_decl),
            Statement::ExpressionStmt(expr) => self.check_expression(expr),
            Statement::ReturnStmt(ret_stmt) => self.check_return_statement(ret_stmt),
        }
    }

    fn check_variable_declaration(&mut self, var_decl: &VariableDecl) -> Result<(), String> {
        if let Some(init_expr) = &var_decl.initializer {
            let init_type = self.check_expression(init_expr)?;
            
            if let Some(expected_type) = &var_decl.r#type {
                // Проверяем совместимость типов
                if !self.is_compatible(&init_type, expected_type) {
                    return Err(format!(
                        "Type mismatch: expected {:?}, found {:?}", 
                        expected_type, init_type
                    ));
                }
            }
        }
        
        Ok(())
    }

    fn check_function_declaration(&mut self, func_decl: &FunctionDecl) -> Result<(), String> {
        // Временно добавляем функцию в таблицу символов
        self.symbol_table.enter_scope();
        
        // Добавляем параметры в таблицу символов
        for param in &func_decl.params {
            let symbol = crate::Symbol {
                name: param.name.clone(),
                r#type: param.r#type.clone(),
                scope_level: self.symbol_table.current_scope_level(),
            };
            self.symbol_table.define(symbol)?;
        }
        
        // Проверяем тело функции
        for stmt in &func_decl.body {
            self.check_statement(stmt)?;
        }
        
        self.symbol_table.exit_scope();
        Ok(())
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<types::Type, String> {
        match expr {
            Expression::Identifier(name) => {
                if let Some(symbol) = self.symbol_table.lookup(name) {
                    Ok(symbol.r#type.clone())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            Expression::NumberLiteral(_) => Ok(types::Type::Int),
            Expression::BinaryOp(bin_op) => self.check_binary_operation(bin_op),
            Expression::FunctionCall(call) => self.check_function_call(call),
        }
    }

    fn check_binary_operation(&mut self, bin_op: &BinaryOpExpr) -> Result<types::Type, String> {
        let left_type = self.check_expression(&bin_op.left)?;
        let right_type = self.check_expression(&bin_op.right)?;
        
        if !self.is_compatible(&left_type, &right_type) {
            return Err("Incompatible types in binary operation".to_string());
        }
        
        // Для большинства операций тип результата такой же, как у операндов
        Ok(left_type)
    }

    fn check_function_call(&mut self, call: &FunctionCallExpr) -> Result<types::Type, String> {
        // Проверка типа функции
        let func_type = self.check_expression(&call.function)?;
        
        // Проверка аргументов
        for arg in &call.arguments {
            self.check_expression(arg)?;
        }
        
        // Возвращаем тип возвращаемого значения функции
        // (в реальности это было бы более сложным)
        Ok(func_type)
    }

    fn check_return_statement(&mut self, ret_stmt: &ReturnStmt) -> Result<(), String> {
        if let Some(value) = &ret_stmt.value {
            self.check_expression(value)?;
        }
        Ok(())
    }

    fn is_compatible(&self, left: &types::Type, right: &types::Type) -> bool {
        // Упрощенная проверка совместимости типов
        left == right
    }
}

// Метод для получения текущего уровня области видимости (добавим его в SymbolTable)
impl SymbolTable {
    pub fn current_scope_level(&self) -> usize {
        self.scope_level
    }
}