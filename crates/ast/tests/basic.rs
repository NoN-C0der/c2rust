use ast::{Program, Statement, Expression, AstSerializer};

#[test]
fn test_ast_serialization() {
    let program = Program {
        statements: vec![],
    };
    
    let serializer = AstSerializer::new();
    let result = serializer.serialize_program(&program);
    
    assert!(result.is_empty());
}