use parser::Parser;

#[test]
fn test_basic_parsing() {
    let tokens = vec![
        lexer::Token::new_identifier("test".to_string()),
        lexer::Token::Eof,
    ];
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_ok());
}