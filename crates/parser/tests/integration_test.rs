//! Интеграционные тесты для парсера

use lexer::Lexer;
use parser::{Parser, ASTNode};

#[test]
fn test_basic_parsing() {
    let input = "int main() { return 0; }";
    let mut lexer = Lexer::new(input);
    
    // Сначала получим токены
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token().expect("Failed to get token");
        tokens.push(token.clone());
        if matches!(token, lexer::Token::Eof) {
            break;
        }
    }
    
    // Теперь передаем токены парсеру
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    // Проверяем, что результат успешно получен
    assert!(result.is_ok());
    
    println!("Basic parsing test passed!");
}