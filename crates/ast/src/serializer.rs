//! Сериализация AST в различные форматы

use crate::node::{ASTNode, Expr, Statement, Type, Parameter, BinaryOperator, UnaryOperator};

pub trait ASTSerializer {
    type Output;
    
    fn serialize(&mut self, ast: &ASTNode) -> Self::Output;
}

pub struct JSONSerializer;

impl JSONSerializer {
    pub fn new() -> Self {
        JSONSerializer
    }
    
    pub fn serialize_to_json(&mut self, ast: &ASTNode) -> String {
        match ast {
            ASTNode::Program(nodes) => {
                let children: Vec<String> = nodes.iter()
                    .map(|node| self.serialize_to_json(node))
                    .collect();
                format!("{{\"type\":\"Program\",\"children\":[{}]}}", children.join(","))
            }
            ASTNode::VariableDeclaration { name, data_type, initializer } => {
                let type_str = self.type_to_json(data_type);
                let init_str = match initializer {
                    Some(expr) => format!("\"initializer\":{}", self.expr_to_json(expr)),
                    None => "\"initializer\":null".to_string(),
                };
                format!("{{\"type\":\"VariableDeclaration\",\"name\":\"{}\",\"dataType\":{},{} }}", name, type_str, init_str)
            }
            ASTNode::FunctionDeclaration { name, parameters, return_type, body } => {
                let params: Vec<String> = parameters.iter()
                    .map(|param| format!("{{\"name\":\"{}\",\"type\":{}}}", param.name, self.type_to_json(&param.param_type)))
                    .collect();
                let ret_type = self.type_to_json(return_type);
                let body_str: Vec<String> = body.iter()
                    .map(|stmt| self.stmt_to_json(stmt))
                    .collect();
                
                format!("{{\"type\":\"FunctionDeclaration\",\"name\":\"{}\",\"parameters\":[{}],\"returnType\":{},\"body\":[{}]}}", 
                    name, params.join(","), ret_type, body_str.join(","))
            }
            ASTNode::Expression(expr) => self.expr_to_json(expr),
            ASTNode::Statement(stmt) => self.stmt_to_json(stmt),
        }
    }
    
    fn type_to_json(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "\"Int\"".to_string(),
            Type::Char => "\"Char\"".to_string(),
            Type::Float => "\"Float\"".to_string(),
            Type::Double => "\"Double\"".to_string(),
            Type::Bool => "\"Bool\"".to_string(),
            Type::Pointer(inner) => format!("{{\"type\":\"Pointer\",\"inner\":{}}}", self.type_to_json(inner)),
            Type::Array(inner, size) => format!("{{\"type\":\"Array\",\"inner\":{},\"size\":{}}}", self.type_to_json(inner), size),
            Type::Custom(name) => format!("{{\"type\":\"Custom\",\"name\":\"{}\"}}", name),
        }
    }
    
    fn expr_to_json(&self, expr: &Expr) -> String {
        match expr {
            Expr::Identifier(name) => format!("{{\"type\":\"Identifier\",\"name\":\"{}\"}}", name),
            Expr::IntegerLiteral(value) => format!("{{\"type\":\"IntegerLiteral\",\"value\":{}}}", value),
            Expr::StringLiteral(value) => format!("{{\"type\":\"StringLiteral\",\"value\":\"{}\"}}", value),
            Expr::BinaryOp { left, operator, right } => {
                let op_str = match operator {
                    BinaryOperator::Add => "Add",
                    BinaryOperator::Sub => "Sub",
                    BinaryOperator::Mul => "Mul",
                    BinaryOperator::Div => "Div",
                    BinaryOperator::Mod => "Mod",
                    BinaryOperator::Eq => "Eq",
                    BinaryOperator::Ne => "Ne",
                    BinaryOperator::Lt => "Lt",
                    BinaryOperator::Le => "Le",
                    BinaryOperator::Gt => "Gt",
                    BinaryOperator::Ge => "Ge",
                    BinaryOperator::And => "And",
                    BinaryOperator::Or => "Or",
                };
                
                format!("{{\"type\":\"BinaryOp\",\"operator\":\"{}\",\"left\":{},\"right\":{}}}", 
                    op_str, self.expr_to_json(left), self.expr_to_json(right))
            }
            Expr::UnaryOp { operator, operand } => {
                let op_str = match operator {
                    UnaryOperator::Neg => "Neg",
                    UnaryOperator::Not => "Not",
                    UnaryOperator::AddrOf => "AddrOf",
                    UnaryOperator::Deref => "Deref",
                };
                
                format!("{{\"type\":\"UnaryOp\",\"operator\":\"{}\",\"operand\":{}}}", 
                    op_str, self.expr_to_json(operand))
            }
            Expr::FunctionCall { name, arguments } => {
                let args: Vec<String> = arguments.iter()
                    .map(|arg| self.expr_to_json(arg))
                    .collect();
                    
                format!("{{\"type\":\"FunctionCall\",\"name\":\"{}\",\"arguments\":[{}]}}", 
                    name, args.join(","))
            }
        }
    }
    
    fn stmt_to_json(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::Return(expr) => {
                match expr {
                    Some(e) => format!("{{\"type\":\"Return\",\"expression\":{}}}", self.expr_to_json(e)),
                    None => "{\"type\":\"Return\",\"expression\":null}".to_string(),
                }
            }
            Statement::Expression(expr) => format!("{{\"type\":\"ExpressionStatement\",\"expression\":{}}}", self.expr_to_json(expr)),
            Statement::Block(stmts) => {
                let stmts_str: Vec<String> = stmts.iter()
                    .map(|stmt| self.stmt_to_json(stmt))
                    .collect();
                    
                format!("{{\"type\":\"Block\",\"statements\":[{}]}}", stmts_str.join(","))
            }
            Statement::If { condition, then_branch, else_branch } => {
                let cond_str = self.expr_to_json(condition);
                let then_str: Vec<String> = then_branch.iter()
                    .map(|stmt| self.stmt_to_json(stmt))
                    .collect();
                    
                match else_branch {
                    Some(else_stmts) => {
                        let else_str: Vec<String> = else_stmts.iter()
                            .map(|stmt| self.stmt_to_json(stmt))
                            .collect();
                            
                        format!("{{\"type\":\"If\",\"condition\":{},\"thenBranch\":[{}],\"elseBranch\":[{}]}}", 
                            cond_str, then_str.join(","), else_str.join(","))
                    }
                    None => {
                        format!("{{\"type\":\"If\",\"condition\":{},\"thenBranch\":[{}]}}", 
                            cond_str, then_str.join(","))
                    }
                }
            }
            Statement::While { condition, body } => {
                let cond_str = self.expr_to_json(condition);
                let body_str: Vec<String> = body.iter()
                    .map(|stmt| self.stmt_to_json(stmt))
                    .collect();
                    
                format!("{{\"type\":\"While\",\"condition\":{},\"body\":[{}]}}", 
                    cond_str, body_str.join(","))
            }
        }
    }
}

impl ASTSerializer for JSONSerializer {
    type Output = String;
    
    fn serialize(&mut self, ast: &ASTNode) -> Self::Output {
        self.serialize_to_json(ast)
    }
}

pub struct TextSerializer;

impl TextSerializer {
    pub fn new() -> Self {
        TextSerializer
    }
    
    pub fn serialize_to_text(&mut self, ast: &ASTNode) -> String {
        match ast {
            ASTNode::Program(nodes) => {
                let mut result = String::from("=== PROGRAM ===\n");
                for node in nodes {
                    result.push_str(&self.node_to_text(node));
                    result.push('\n');
                }
                result.push_str("=== END PROGRAM ===\n");
                result
            }
            ASTNode::VariableDeclaration { name, data_type, initializer } => {
                let type_str = self.type_to_text(data_type);
                let init_str = match initializer {
                    Some(expr) => format!(" = {}", self.expr_to_text(expr)),
                    None => String::new(),
                };
                format!("VAR_DECL: {} : {}{}\n", name, type_str, init_str)
            }
            ASTNode::FunctionDeclaration { name, parameters, return_type, body } => {
                let params_str = parameters
                    .iter()
                    .map(|param| format!("{}: {}", param.name, self.type_to_text(&param.param_type)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret_type = self.type_to_text(return_type);
                
                let mut result = format!("FUNC_DECL: {}({}) -> {}\n", name, params_str, ret_type);
                result.push_str("{\n");
                for stmt in body {
                    result.push_str(&format!("  {}\n", self.stmt_to_text(stmt).replace("\n", "\n  ")));
                }
                result.push_str("}\n");
                result
            }
            ASTNode::Expression(expr) => format!("EXPR: {}\n", self.expr_to_text(expr)),
            ASTNode::Statement(stmt) => self.stmt_to_text(stmt),
        }
    }
    
    fn node_to_text(&mut self, node: &ASTNode) -> String {
        self.serialize_to_text(node)
    }
    
    fn type_to_text(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Pointer(inner) => format!("ptr({})", self.type_to_text(inner)),
            Type::Array(inner, size) => format!("array({}, {})", self.type_to_text(inner), size),
            Type::Custom(name) => name.clone(),
        }
    }
    
    fn expr_to_text(&self, expr: &Expr) -> String {
        match expr {
            Expr::Identifier(name) => name.clone(),
            Expr::IntegerLiteral(value) => value.to_string(),
            Expr::StringLiteral(value) => format!("\"{}\"", value),
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
                
                format!("({} {} {})", 
                    self.expr_to_text(left), 
                    op_str, 
                    self.expr_to_text(right))
            }
            Expr::UnaryOp { operator, operand } => {
                let op_str = match operator {
                    UnaryOperator::Neg => "-",
                    UnaryOperator::Not => "!",
                    UnaryOperator::AddrOf => "&",
                    UnaryOperator::Deref => "*",
                };
                
                format!("({}{})", op_str, self.expr_to_text(operand))
            }
            Expr::FunctionCall { name, arguments } => {
                let args_str = arguments
                    .iter()
                    .map(|arg| self.expr_to_text(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                    
                format!("{}({})", name, args_str)
            }
        }
    }
    
    fn stmt_to_text(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::Return(expr) => {
                match expr {
                    Some(e) => format!("RETURN: {}\n", self.expr_to_text(e)),
                    None => "RETURN\n".to_string(),
                }
            }
            Statement::Expression(expr) => format!("EXPR_STMT: {}\n", self.expr_to_text(expr)),
            Statement::Block(stmts) => {
                let mut result = String::from("{\n");
                for stmt in stmts {
                    result.push_str(&format!("  {}\n", self.stmt_to_text(stmt).replace("\n", "\n  ")));
                }
                result.push_str("}\n");
                result
            }
            Statement::If { condition, then_branch, else_branch } => {
                let cond_str = self.expr_to_text(condition);
                let then_str = self.statements_to_text(then_branch, 1);
                
                match else_branch {
                    Some(else_stmts) => {
                        let else_str = self.statements_to_text(else_stmts, 1);
                        format!("IF ({}) {{\n{}}} ELSE {{\n{}}}\n", cond_str, then_str, else_str)
                    }
                    None => {
                        format!("IF ({}) {{\n{}}}\n", cond_str, then_str)
                    }
                }
            }
            Statement::While { condition, body } => {
                let cond_str = self.expr_to_text(condition);
                let body_str = self.statements_to_text(body, 1);
                format!("WHILE ({}) {{\n{}}}\n", cond_str, body_str)
            }
        }
    }
    
    fn statements_to_text(&self, stmts: &[Statement], indent: usize) -> String {
        let mut result = String::new();
        let prefix = "  ".repeat(indent);
        
        for stmt in stmts {
            let stmt_text = self.stmt_to_text(stmt).trim_end().to_string();
            for line in stmt_text.lines() {
                result.push_str(&format!("{}{}\n", prefix, line));
            }
        }
        
        result
    }
}

impl ASTSerializer for TextSerializer {
    type Output = String;
    
    fn serialize(&mut self, ast: &ASTNode) -> Self::Output {
        self.serialize_to_text(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{ASTNode, Expr, BinaryOperator};

    #[test]
    fn test_json_serialization() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        
        let node = ASTNode::Expression(expr);
        let mut serializer = JSONSerializer::new();
        let result = serializer.serialize(&node);
        
        assert!(result.contains("BinaryOp"));
        assert!(result.contains("Add"));
    }

    #[test]
    fn test_text_serialization() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: BinaryOperator::Add,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        
        let node = ASTNode::Expression(expr);
        let mut serializer = TextSerializer::new();
        let result = serializer.serialize(&node);
        
        assert!(result.contains("(a + b)"));
    }
}