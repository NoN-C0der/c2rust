use lexer::{Lexer, Token};

#[test]
fn test_basic_tokenization() {
    let mut lexer = Lexer::new("hello world".to_string());
    let tokens = lexer.tokenize().expect("Failed to tokenize");
    
    assert_eq!(tokens.len(), 3); // hello, world, Eof
    match &tokens[0] {
        Token::Identifier(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected identifier"),
    }
}