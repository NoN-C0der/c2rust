//! Интеграционные тесты для лексера

use lexer::{Lexer, Token};

#[test]
fn test_complex_tokenization() {
    let input = r#"
        int main() {
            int x = 42;
            char* str = "hello world";
            return 0;
        }
    "#;
    
    let mut lexer = Lexer::new(input);
    
    // Пропустить начальные пробелы и переносы строк
    assert!(matches!(lexer.next_token().unwrap(), Token::Identifier(ref id) if id == "int"));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("main".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("(".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier(")".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("{".to_string()));
    
    println!("Integration test passed!");
}