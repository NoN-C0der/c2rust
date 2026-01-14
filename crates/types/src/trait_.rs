//! Определение трейтов

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<TraitMethod>,
    pub associated_types: Vec<String>,
    pub super_traits: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub name: String,
    pub signature: String, // Упрощенная сигнатура метода
    pub required: bool,    // Является ли обязательным для реализации
}

impl Trait {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: Vec::new(),
            associated_types: Vec::new(),
            super_traits: Vec::new(),
        }
    }

    pub fn add_method(mut self, method: TraitMethod) -> Self {
        self.methods.push(method);
        self
    }

    pub fn add_associated_type(mut self, name: String) -> Self {
        self.associated_types.push(name);
        self
    }

    pub fn add_super_trait(mut self, name: String) -> Self {
        self.super_traits.push(name);
        self
    }
}

#[derive(Debug, Clone)]
pub struct TraitImpl {
    pub trait_name: String,
    pub for_type: Type, // Используем Type из модуля type_
    pub methods: Vec<ImplementationMethod>,
}

#[derive(Debug, Clone)]
pub struct ImplementationMethod {
    pub name: String,
    pub implementation: String, // Тело метода или указание на реализацию
}

pub struct TraitEnvironment {
    traits: HashMap<String, Trait>,
    implementations: Vec<TraitImpl>,
}

impl TraitEnvironment {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
            implementations: Vec::new(),
        }
    }

    pub fn register_trait(&mut self, trait_: Trait) {
        self.traits.insert(trait_.name.clone(), trait_);
    }

    pub fn implement_trait(&mut self, impl_: TraitImpl) {
        self.implementations.push(impl_);
    }

    pub fn find_trait(&self, name: &str) -> Option<&Trait> {
        self.traits.get(name)
    }

    pub fn find_implementations_for_type(&self, type_name: &str) -> Vec<&TraitImpl> {
        self.implementations
            .iter()
            .filter(|imp| match &imp.for_type {
                Type::Struct(name) | Type::Enum(name) => name == type_name,
                _ => false,
            })
            .collect()
    }
}

// Используем Type из нашего модуля
use crate::Type;