//! Грамматика языка C++

/// Категории грамматических правил
pub mod rules {
    use std::collections::HashMap;

    /// Определение нетерминала грамматики
    #[derive(Debug, Clone, PartialEq)]
    pub struct NonTerminal {
        pub name: String,
        pub description: String,
    }

    /// Определение терминала грамматики
    #[derive(Debug, Clone, PartialEq)]
    pub struct Terminal {
        pub token_type: String,
        pub representation: String,
    }

    /// Правило грамматики
    #[derive(Debug, Clone, PartialEq)]
    pub struct GrammarRule {
        pub left_side: String,      // Левая часть правила (нетерминал)
        pub right_side: Vec<String>, // Правая часть правила (последовательность символов)
        pub semantic_action: Option<String>, // Семантическое действие
    }

    impl GrammarRule {
        pub fn new(left: &str, right: Vec<&str>) -> Self {
            GrammarRule {
                left_side: left.to_string(),
                right_side: right.iter().map(|s| s.to_string()).collect(),
                semantic_action: None,
            }
        }
    }

    /// Полная грамматика языка
    #[derive(Debug)]
    pub struct Grammar {
        pub terminals: Vec<Terminal>,
        pub non_terminals: Vec<NonTerminal>,
        pub rules: Vec<GrammarRule>,
        pub start_symbol: String,
    }

    impl Grammar {
        pub fn new() -> Self {
            Grammar {
                terminals: vec![],
                non_terminals: vec![],
                rules: vec![],
                start_symbol: "Program".to_string(),
            }
        }

        pub fn add_rule(&mut self, rule: GrammarRule) {
            self.rules.push(rule);
        }

        pub fn get_rules_for_non_terminal(&self, non_terminal: &str) -> Vec<&GrammarRule> {
            self.rules
                .iter()
                .filter(|rule| rule.left_side == non_terminal)
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rules::*;

    #[test]
    fn test_grammar_creation() {
        let mut grammar = Grammar::new();
        let rule = GrammarRule::new("Statement", vec!["Expression", ";"]);
        
        grammar.add_rule(rule);
        assert_eq!(grammar.rules.len(), 1);
        assert_eq!(grammar.start_symbol, "Program");
    }
}