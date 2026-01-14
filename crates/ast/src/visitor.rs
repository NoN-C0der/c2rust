//! Посетитель AST

use crate::node::*;

pub trait AstVisitor {
    /// Посещает программу
    fn visit_program(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.visit_statement(stmt);
        }
    }

    /// Посещает утверждение
    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration(var_decl) => self.visit_variable_declaration(var_decl),
            Statement::FunctionDeclaration(func_decl) => self.visit_function_declaration(func_decl),
            Statement::ExpressionStmt(expr) => self.visit_expression(expr),
            Statement::ReturnStmt(ret_stmt) => self.visit_return_statement(ret_stmt),
        }
    }

    /// Посещает объявление переменной
    fn visit_variable_declaration(&mut self, var_decl: &VariableDecl) {
        if let Some(init) = &var_decl.initializer {
            self.visit_expression(init);
        }
    }

    /// Посещает объявление функции
    fn visit_function_declaration(&mut self, func_decl: &FunctionDecl) {
        for param in &func_decl.params {
            self.visit_parameter(param);
        }
        for stmt in &func_decl.body {
            self.visit_statement(stmt);
        }
    }

    /// Посещает параметр
    fn visit_parameter(&mut self, param: &Parameter) {
        // Пока пусто
    }

    /// Посещает выражение
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(_) => {},
            Expression::NumberLiteral(_) => {},
            Expression::BinaryOp(bin_op) => self.visit_binary_op(bin_op),
            Expression::FunctionCall(call) => self.visit_function_call(call),
        }
    }

    /// Посещает бинарную операцию
    fn visit_binary_op(&mut self, bin_op: &BinaryOpExpr) {
        self.visit_expression(&bin_op.left);
        self.visit_expression(&bin_op.right);
    }

    /// Посещает вызов функции
    fn visit_function_call(&mut self, call: &FunctionCallExpr) {
        self.visit_expression(&call.function);
        for arg in &call.arguments {
            self.visit_expression(arg);
        }
    }

    /// Посещает оператор возврата
    fn visit_return_statement(&mut self, ret_stmt: &ReturnStmt) {
        if let Some(value) = &ret_stmt.value {
            self.visit_expression(value);
        }
    }
}

/// Пример конкретной реализации посетителя
pub struct PrintVisitor {
    indent_level: usize,
}

impl PrintVisitor {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }
}

impl AstVisitor for PrintVisitor {
    fn visit_program(&mut self, program: &Program) {
        println!("Program {{");
        self.indent_level += 1;
        for stmt in &program.statements {
            self.visit_statement(stmt);
        }
        self.indent_level -= 1;
        println!("{}}}", self.indent(""));
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration(var_decl) => {
                println!("{}VariableDeclaration: {}", self.indent(), var_decl.name);
            },
            Statement::FunctionDeclaration(func_decl) => {
                println!("{}FunctionDeclaration: {}", self.indent(), func_decl.name);
            },
            Statement::ExpressionStmt(expr) => {
                println!("{}ExpressionStmt:", self.indent());
                self.visit_expression(expr);
            },
            Statement::ReturnStmt(ret_stmt) => {
                println!("{}ReturnStmt:", self.indent());
                self.visit_return_statement(ret_stmt);
            },
        }
    }
}