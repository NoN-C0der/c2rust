//! Разрешение имен

use crate::{SymbolTable, Symbol};
use ast::node::*;

pub struct NameResolver {
    symbol_table: SymbolTable,
}

impl NameResolver {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn resolve_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.resolve_statement(stmt)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VariableDeclaration(var_decl) => self.resolve_variable_declaration(var_decl),
            Statement::FunctionDeclaration(func_decl) => self.resolve_function_declaration(func_decl),
            Statement::ExpressionStmt(expr) => self.resolve_expression(expr),
            Statement::ReturnStmt(ret_stmt) => self.resolve_return_statement(ret_stmt),
        }
    }

    fn resolve_variable_declaration(&mut self, var_decl: &VariableDecl) -> Result<(), String> {
        // Если есть инициализатор, разрешаем имена в нем
        if let Some(init_expr) = &var_decl.initializer {
            self.resolve_expression(init_expr)?;
        }
        
        // Добавляем переменную в таблицу символов
        let symbol = Symbol {
            name: var_decl.name.clone(),
            r#type: var_decl.r#type.clone().unwrap_or(types::Type::Int), // временно
            scope_level: self.symbol_table.current_scope_level(),
        };
        
        self.symbol_table.define(symbol)
    }

    fn resolve_function_declaration(&mut self, func_decl: &FunctionDecl) -> Result<(), String> {
        self.symbol_table.enter_scope();
        
        // Добавляем параметры в таблицу символов
        for param in &func_decl.params {
            let symbol = Symbol {
                name: param.name.clone(),
                r#type: param.r#type.clone(),
                scope_level: self.symbol_table.current_scope_level(),
            };
            self.symbol_table.define(symbol)?;
        }
        
        // Разрешаем имена в теле функции
        for stmt in &func_decl.body {
            self.resolve_statement(stmt)?;
        }
        
        self.symbol_table.exit_scope();
        Ok(())
    }

    fn resolve_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::Identifier(name) => {
                // Проверяем, определена ли переменная
                if self.symbol_table.lookup(name).is_none() {
                    return Err(format!("Undefined variable: {}", name));
                }
                Ok(())
            }
            Expression::NumberLiteral(_) => Ok(()),
            Expression::BinaryOp(bin_op) => {
                self.resolve_expression(&bin_op.left)?;
                self.resolve_expression(&bin_op.right)?;
                Ok(())
            }
            Expression::FunctionCall(call) => {
                // Разрешаем имя функции
                self.resolve_expression(&call.function)?;
                
                // Разрешаем аргументы
                for arg in &call.arguments {
                    self.resolve_expression(arg)?;
                }
                Ok(())
            }
        }
    }

    fn resolve_return_statement(&mut self, ret_stmt: &ReturnStmt) -> Result<(), String> {
        if let Some(value) = &ret_stmt.value {
            self.resolve_expression(value)?;
        }
        Ok(())
    }
}