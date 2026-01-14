use semantic::{SymbolTable, TypeChecker, NameResolver};

#[test]
fn test_symbol_table_creation() {
    let table = SymbolTable::new();
    assert_eq!(table.get_all_symbols().len(), 0);
}

#[test]
fn test_type_checker_creation() {
    let checker = TypeChecker::new();
    // Проверим, что можем создать экземпляр
    assert!(true); // Просто проверка создания
}

#[test]
fn test_name_resolver_creation() {
    let resolver = NameResolver::new();
    // Проверим, что можем создать экземпляр
    assert!(true); // Просто проверка создания
}