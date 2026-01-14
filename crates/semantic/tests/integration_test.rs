//! Интеграционные тесты для семантического анализа

use ast::{ASTNode, Expr, BinaryOperator, Type};
use semantic::{TypeChecker, NameResolver};

#[test]
fn test_semantic_analysis_pipeline() {
    // Создаём простую программу: объявление переменных и выражение
    let x_decl = ASTNode::VariableDeclaration {
        name: "x".to_string(),
        data_type: Type::Int,
        initializer: Some(Box::new(Expr::IntegerLiteral(10))),
    };
    
    let y_decl = ASTNode::VariableDeclaration {
        name: "y".to_string(),
        data_type: Type::Int,
        initializer: Some(Box::new(Expr::IntegerLiteral(20))),
    };
    
    let sum_expr = ASTNode::Expression(Expr::BinaryOp {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOperator::Add,
        right: Box::new(Expr::Identifier("y".to_string())),
    });
    
    let program = ASTNode::Program(vec![x_decl, y_decl, sum_expr]);
    
    // Тестируем проверку типов
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok(), "Type checking should pass for valid program");
    
    // Тестируем разрешение имён
    let mut name_resolver = NameResolver::new();
    let name_resolve_result = name_resolver.resolve_program(&program);
    assert!(name_resolve_result.is_ok(), "Name resolution should pass for valid program");
    
    println!("Semantic analysis pipeline test passed!");
}

#[test]
fn test_type_mismatch_detection() {
    // Создаём программу с потенциальной ошибкой типов
    let int_var = ASTNode::VariableDeclaration {
        name: "num".to_string(),
        data_type: Type::Int,
        initializer: Some(Box::new(Expr::IntegerLiteral(42))),
    };
    
    let str_var = ASTNode::VariableDeclaration {
        name: "text".to_string(),
        data_type: Type::Custom("string".to_string()),
        initializer: Some(Box::new(Expr::StringLiteral("hello".to_string()))),
    };
    
    // Пытаемся сложить число и строку (это должно вызвать ошибку типов)
    let invalid_expr = ASTNode::Expression(Expr::BinaryOp {
        left: Box::new(Expr::Identifier("num".to_string())),
        operator: BinaryOperator::Add,
        right: Box::new(Expr::Identifier("text".to_string())),
    });
    
    let program = ASTNode::Program(vec![int_var, str_var, invalid_expr]);
    
    // Проверяем, что проверка типов обнаруживает ошибку
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    
    // Ожидаем, что будет ошибка
    assert!(type_check_result.is_err(), "Type checking should fail for incompatible types");
    
    println!("Type mismatch detection test passed!");
}