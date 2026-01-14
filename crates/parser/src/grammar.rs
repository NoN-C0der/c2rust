//! Грамматика языка

pub mod rules {
    //! Правила грамматики
    
    pub struct GrammarRule {
        pub name: String,
        pub pattern: String,
    }
    
    impl GrammarRule {
        pub fn new(name: String, pattern: String) -> Self {
            Self { name, pattern }
        }
    }
}

pub mod precedence {
    //! Приоритеты операций
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Precedence {
        Lowest,
        Equals,
        LessGreater,
        Sum,
        Product,
        Prefix,
        Call,
        Index,
    }
    
    impl Precedence {
        pub fn next_higher(&self) -> Option<Self> {
            match self {
                Precedence::Lowest => Some(Precedence::Equals),
                Precedence::Equals => Some(Precedence::LessGreater),
                Precedence::LessGreater => Some(Precedence::Sum),
                Precedence::Sum => Some(Precedence::Product),
                Precedence::Product => Some(Precedence::Prefix),
                Precedence::Prefix => Some(Precedence::Call),
                Precedence::Call => Some(Precedence::Index),
                Precedence::Index => None,
            }
        }
    }
}