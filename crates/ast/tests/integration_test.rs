//! Интеграционные тесты для AST

use ast::{ASTNode, Expr, BinaryOperator, Type, Statement};

#[test]
fn test_ast_serialization() {
    let expr = Expr::BinaryOp {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOperator::Add,
        right: Box::new(Expr::IntegerLiteral(42)),
    };
    
    let node = ASTNode::Expression(expr);
    
    // Тестируем JSON сериализацию
    let mut json_serializer = ast::JSONSerializer::new();
    let json_result = json_serializer.serialize(&node);
    assert!(json_result.contains("BinaryOp"));
    assert!(json_result.contains("Add"));
    
    // Тестируем текстовую сериализацию
    let mut text_serializer = ast::TextSerializer::new();
    let text_result = text_serializer.serialize(&node);
    assert!(text_result.contains("(x + 42)"));
    
    println!("AST serialization test passed!");
}

#[test]
fn test_ast_visitor_pattern() {
    let expr = Expr::BinaryOp {
        left: Box::new(Expr::Identifier("a".to_string())),
        operator: BinaryOperator::Mul,
        right: Box::new(Expr::Identifier("b".to_string())),
    };
    
    let node = ASTNode::Expression(expr);
    
    // Тестируем посетителя
    let mut visitor = ast::PrintVisitor::new();
    let result = visitor.visit_node(&node);
    
    assert!(result.contains("BinaryOp"));
    assert!(result.contains("*"));
    
    println!("AST visitor pattern test passed!");
}