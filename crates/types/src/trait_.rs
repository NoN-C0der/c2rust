//! Типажи (traits) для системы типов C++

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<MethodSignature>,
    pub associated_types: Vec<String>,
    pub required_traits: Vec<String>,  // Типажи, которые должны быть реализованы
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub name: String,
    pub param_types: Vec<String>,
    pub return_type: String,
    pub is_const: bool,
    pub is_virtual: bool,
}

#[derive(Debug, Clone)]
pub struct TraitImplementation {
    pub trait_name: String,
    pub implementing_type: String,
    pub method_implementations: HashMap<String, String>, // метод -> имя функции реализации
    pub associated_type_mappings: HashMap<String, String>, // ассоциированный тип -> конкретный тип
}

pub struct TraitSystem {
    traits: HashMap<String, Trait>,
    implementations: Vec<TraitImplementation>,
}

impl TraitSystem {
    pub fn new() -> Self {
        TraitSystem {
            traits: HashMap::new(),
            implementations: Vec::new(),
        }
    }

    pub fn define_trait(&mut self, trait_def: Trait) -> Result<(), String> {
        if self.traits.contains_key(&trait_def.name) {
            return Err(format!("Trait '{}' already defined", trait_def.name));
        }
        
        self.traits.insert(trait_def.name.clone(), trait_def);
        Ok(())
    }

    pub fn implement_trait(&mut self, impl_def: TraitImplementation) -> Result<(), String> {
        // Проверяем, существует ли типаж
        if !self.traits.contains_key(&impl_def.trait_name) {
            return Err(format!("Trait '{}' does not exist", impl_def.trait_name));
        }
        
        // Проверяем, реализуются ли требуемые типажи
        if let Some(trait_def) = self.traits.get(&impl_def.trait_name) {
            for required_trait in &trait_def.required_traits {
                if !self.has_trait_implementation(&impl_def.implementing_type, required_trait) {
                    return Err(format!(
                        "Type '{}' must implement required trait '{}' before implementing '{}'",
                        impl_def.implementing_type, required_trait, impl_def.trait_name
                    ));
                }
            }
            
            // Проверяем, реализованы ли все необходимые методы
            for method in &trait_def.methods {
                if !impl_def.method_implementations.contains_key(&method.name) {
                    return Err(format!(
                        "Missing implementation for method '{}' in trait '{}'",
                        method.name, impl_def.trait_name
                    ));
                }
            }
        }
        
        self.implementations.push(impl_def);
        Ok(())
    }

    pub fn has_trait_implementation(&self, type_name: &str, trait_name: &str) -> bool {
        self.implementations.iter().any(|imp| {
            imp.implementing_type == type_name && imp.trait_name == trait_name
        })
    }

    pub fn get_trait(&self, trait_name: &str) -> Option<&Trait> {
        self.traits.get(trait_name)
    }

    pub fn get_implementations_for_type(&self, type_name: &str) -> Vec<&TraitImplementation> {
        self.implementations
            .iter()
            .filter(|imp| imp.implementing_type == type_name)
            .collect()
    }

    pub fn get_implementations_of_trait(&self, trait_name: &str) -> Vec<&TraitImplementation> {
        self.implementations
            .iter()
            .filter(|imp| imp.trait_name == trait_name)
            .collect()
    }

    pub fn can_coerce_to_trait(&self, type_name: &str, trait_name: &str) -> bool {
        self.has_trait_implementation(type_name, trait_name)
    }
}

// Вспомогательные типы для C++ специфичных особенностей
#[derive(Debug, Clone, PartialEq)]
pub enum TypeCategory {
    Arithmetic,
    Integral,
    FloatingPoint,
    Signed,
    Unsigned,
    Compound,
    Fundamental,
    POD,  // Plain Old Data
    Trivial,
    StandardLayout,
    Aggregate,
    Polymorphic,
    Abstract,
    Final,
    Reference,
    Pointer,
    MemberPointer,
}

#[derive(Debug, Clone)]
pub struct TypeProperties {
    pub category: TypeCategory,
    pub size: Option<usize>,
    pub alignment: Option<usize>,
    pub is_const: bool,
    pub is_volatile: bool,
    pub is_restrict: bool,  // C99 restrict qualifier
}

impl TypeProperties {
    pub fn new(category: TypeCategory) -> Self {
        TypeProperties {
            category,
            size: None,
            alignment: None,
            is_const: false,
            is_volatile: false,
            is_restrict: false,
        }
    }

    pub fn with_const(mut self) -> Self {
        self.is_const = true;
        self
    }

    pub fn with_volatile(mut self) -> Self {
        self.is_volatile = true;
        self
    }

    pub fn with_restrict(mut self) -> Self {
        self.is_restrict = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_system() {
        let mut system = TraitSystem::new();
        
        // Определяем типаж Printable
        let printable_trait = Trait {
            name: "Printable".to_string(),
            methods: vec![MethodSignature {
                name: "print".to_string(),
                param_types: vec![],
                return_type: "void".to_string(),
                is_const: true,
                is_virtual: true,
            }],
            associated_types: vec![],
            required_traits: vec![],
        };
        
        system.define_trait(printable_trait).unwrap();
        
        // Проверяем, что типаж существует
        assert!(system.get_trait("Printable").is_some());
        
        // Реализуем типаж для типа Int
        let impl_def = TraitImplementation {
            trait_name: "Printable".to_string(),
            implementing_type: "Int".to_string(),
            method_implementations: vec![("print".to_string(), "int_print".to_string())]
                .into_iter()
                .collect(),
            associated_type_mappings: HashMap::new(),
        };
        
        let result = system.implement_trait(impl_def);
        assert!(result.is_ok());
        
        // Проверяем, что реализация существует
        assert!(system.has_trait_implementation("Int", "Printable"));
    }

    #[test]
    fn test_type_properties() {
        let props = TypeProperties::new(TypeCategory::Arithmetic)
            .with_const();
        
        assert_eq!(props.category, TypeCategory::Arithmetic);
        assert!(props.is_const);
    }
}