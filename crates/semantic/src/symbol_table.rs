//! Таблица символов

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub r#type: types::Type,
    pub scope_level: usize,
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    scope_level: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            scope_level: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scope_level += 1;
    }

    pub fn exit_scope(&mut self) {
        // Удаляем символы текущего уровня области видимости
        self.symbols.retain(|_, sym| sym.scope_level < self.scope_level);
        if self.scope_level > 0 {
            self.scope_level -= 1;
        }
    }

    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        if self.symbols.contains_key(&symbol.name) {
            return Err(format!("Symbol '{}' already defined in current scope", symbol.name));
        }
        
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        let symbol = Symbol {
            name: "test".to_string(),
            r#type: types::Type::Int,
            scope_level: 0,
        };
        
        assert!(table.define(symbol).is_ok());
        assert!(table.lookup("test").is_some());
    }
}