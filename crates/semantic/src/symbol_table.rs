//! Таблица символов для семантического анализа

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope_level: usize,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable(String),           // Имя типа переменной
    Function(FunctionSignature), // Подпись функции
    TypeDefinition(String),     // Определение типа
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub return_type: String,
    pub parameters: Vec<(String, String)>,  // (имя параметра, тип параметра)
}

pub struct SymbolTable {
    symbols: HashMap<String, Vec<Symbol>>,  // Имя -> список символов (для разных областей видимости)
    current_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            current_scope: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope += 1;
    }

    pub fn exit_scope(&mut self) {
        // Удаляем все символы текущего уровня
        for symbols in self.symbols.values_mut() {
            symbols.retain(|symbol| symbol.scope_level < self.current_scope);
        }
        if self.current_scope > 0 {
            self.current_scope -= 1;
        }
    }

    pub fn define_variable(&mut self, name: String, var_type: String, mutable: bool) -> Result<(), String> {
        let symbol = Symbol {
            name: name.clone(),
            symbol_type: SymbolType::Variable(var_type),
            scope_level: self.current_scope,
            mutable,
        };

        self.symbols.entry(name).or_insert_with(Vec::new).push(symbol);
        Ok(())
    }

    pub fn define_function(&mut self, name: String, signature: FunctionSignature) -> Result<(), String> {
        let symbol = Symbol {
            name: name.clone(),
            symbol_type: SymbolType::Function(signature),
            scope_level: self.current_scope,
            mutable: false,  // Функции неизменяемы
        };

        self.symbols.entry(name).or_insert_with(Vec::new).push(symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
            .and_then(|symbols| symbols.last())  // Берем последний символ (наиболее вложенный)
    }

    pub fn lookup_in_current_scope(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
            .and_then(|symbols| {
                symbols.iter()
                    .rev()
                    .find(|symbol| symbol.scope_level == self.current_scope)
            })
    }

    pub fn lookup_all_scopes(&self, name: &str) -> Option<&[Symbol]> {
        self.symbols.get(name).map(|v| v.as_slice())
    }

    pub fn current_scope(&self) -> usize {
        self.current_scope
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table_basics() {
        let mut table = SymbolTable::new();
        
        // Добавляем переменную
        table.define_variable("x".to_string(), "int".to_string(), true).unwrap();
        
        // Проверяем, что переменная найдена
        let sym = table.lookup("x").unwrap();
        assert_eq!(sym.name, "x");
        assert!(sym.mutable);
        assert!(matches!(sym.symbol_type, SymbolType::Variable(ref t) if t == "int"));
        
        // Входим в новую область видимости
        table.enter_scope();
        table.define_variable("y".to_string(), "string".to_string(), false).unwrap();
        
        // Проверяем, что в новой области видимости есть переменная
        let sym_y = table.lookup("y").unwrap();
        assert_eq!(sym_y.name, "y");
        assert_eq!(sym_y.scope_level, 1);
        
        // Выходим из области видимости
        table.exit_scope();
        
        // Переменная y больше не доступна
        assert!(table.lookup("y").is_none());
        // Переменная x всё ещё доступна
        assert!(table.lookup("x").is_some());
    }

    #[test]
    fn test_function_definition() {
        let mut table = SymbolTable::new();
        
        let signature = FunctionSignature {
            return_type: "void".to_string(),
            parameters: vec![("x".to_string(), "int".to_string())],
        };
        
        table.define_function("my_func".to_string(), signature).unwrap();
        
        let sym = table.lookup("my_func").unwrap();
        assert!(matches!(sym.symbol_type, SymbolType::Function(_)));
    }
}