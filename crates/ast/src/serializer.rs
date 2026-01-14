//! Сериализация AST

use crate::node::*;

pub struct AstSerializer;

impl AstSerializer {
    pub fn new() -> Self {
        Self
    }

    /// Сериализует программу в строку
    pub fn serialize_program(&self, program: &Program) -> String {
        let mut result = String::new();
        for stmt in &program.statements {
            result.push_str(&self.serialize_statement(stmt));
            result.push('\n');
        }
        result
    }

    /// Сериализует утверждение
    fn serialize_statement(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::VariableDeclaration(var_decl) => self.serialize_variable_declaration(var_decl),
            Statement::FunctionDeclaration(func_decl) => self.serialize_function_declaration(func_decl),
            Statement::ExpressionStmt(expr) => format!("{};", self.serialize_expression(expr)),
            Statement::ReturnStmt(ret_stmt) => self.serialize_return_statement(ret_stmt),
        }
    }

    /// Сериализует объявление переменной
    fn serialize_variable_declaration(&self, var_decl: &VariableDecl) -> String {
        let type_str = match &var_decl.r#type {
            Some(ty) => format!("{}", self.serialize_type(ty)),
            None => "".to_string(),
        };
        let init_str = match &var_decl.initializer {
            Some(init) => format!(" = {}", self.serialize_expression(init)),
            None => "".to_string(),
        };
        format!("let {}{}{}", var_decl.name, type_str, init_str)
    }

    /// Сериализует объявление функции
    fn serialize_function_declaration(&self, func_decl: &FunctionDecl) -> String {
        let params_str = func_decl
            .params
            .iter()
            .map(|param| format!("{}: {}", param.name, self.serialize_type(&param.r#type)))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_type_str = match &func_decl.return_type {
            Some(ret_ty) => format!(" -> {}", self.serialize_type(ret_ty)),
            None => "".to_string(),
        };

        let body_str = func_decl
            .body
            .iter()
            .map(|stmt| format!("  {}", self.serialize_statement(stmt).replace("\n", "\n  ")))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "fn {}({}){} {{\n{}\n}}",
            func_decl.name, params_str, return_type_str, body_str
        )
    }

    /// Сериализует выражение
    fn serialize_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(name) => name.clone(),
            Expression::NumberLiteral(n) => n.to_string(),
            Expression::BinaryOp(bin_op) => self.serialize_binary_op(bin_op),
            Expression::FunctionCall(call) => self.serialize_function_call(call),
        }
    }

    /// Сериализует бинарную операцию
    fn serialize_binary_op(&self, bin_op: &BinaryOpExpr) -> String {
        let op_str = match bin_op.operator {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::Eq => "==",
            BinaryOperator::NotEq => "!=",
            BinaryOperator::Lt => "<",
            BinaryOperator::Gt => ">",
            BinaryOperator::LtEq => "<=",
            BinaryOperator::GtEq => ">=",
        };
        format!(
            "({} {} {})",
            self.serialize_expression(&bin_op.left),
            op_str,
            self.serialize_expression(&bin_op.right)
        )
    }

    /// Сериализует вызов функции
    fn serialize_function_call(&self, call: &FunctionCallExpr) -> String {
        let args_str = call
            .arguments
            .iter()
            .map(|arg| self.serialize_expression(arg))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}({})", self.serialize_expression(&call.function), args_str)
    }

    /// Сериализует оператор возврата
    fn serialize_return_statement(&self, ret_stmt: &ReturnStmt) -> String {
        match &ret_stmt.value {
            Some(value) => format!("return {}", self.serialize_expression(value)),
            None => "return".to_string(),
        }
    }

    /// Сериализует тип
    fn serialize_type(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::Pointer(inner) => format!("*{}", self.serialize_type(inner)),
            Type::Array(inner, size) => format!("[{}; {}]", self.serialize_type(inner), size),
        }
    }
}