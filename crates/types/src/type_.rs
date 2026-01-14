//! Определение базовых типов данных

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Примитивные типы
    Void,
    Bool,
    Char,
    Int,
    Float,
    Double,
    Long,
    Short,
    Signed,
    Unsigned,

    /// Составные типы
    Pointer(Box<Type>),
    Reference(Box<Type>),
    Array(Box<Type>, usize),
    Function {
        return_type: Box<Type>,
        param_types: Vec<Type>,
    },

    /// Пользовательские типы
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Union {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Enum {
        name: String,
        variants: Vec<String>,
    },

    /// Шаблонные типы
    TemplateInstance {
        template_name: String,
        args: Vec<Type>,
    },

    /// Специальные типы
    Auto,
    Decltype(String),
    Complex,
    Imaginary,
}

impl Type {
    pub fn size_of(&self) -> Option<usize> {
        match self {
            Type::Void => Some(0),
            Type::Bool => Some(1),
            Type::Char => Some(1),
            Type::Int => Some(4),
            Type::Float => Some(4),
            Type::Double => Some(8),
            Type::Long => Some(8),
            Type::Short => Some(2),
            Type::Signed => Some(4), // зависит от целевого типа
            Type::Unsigned => Some(4), // зависит от целевого типа
            Type::Pointer(_) => Some(8), // указатель 64 бита
            Type::Reference(_) => Some(8), // ссылка представлена как указатель
            Type::Array(element_type, count) => {
                element_type.size_of().map(|size| size * count)
            }
            Type::Struct { fields, .. } => {
                let mut total_size = 0;
                for (_, field_type) in fields {
                    if let Some(field_size) = field_type.size_of() {
                        total_size += field_size;
                    } else {
                        return None; // Не можем вычислить размер
                    }
                }
                Some(total_size)
            }
            Type::Union { fields, .. } => {
                let mut max_size = 0;
                for (_, field_type) in fields {
                    if let Some(field_size) = field_type.size_of() {
                        max_size = max_size.max(field_size);
                    } else {
                        return None; // Не можем вычислить размер
                    }
                }
                Some(max_size)
            }
            Type::Enum { .. } => Some(4), // обычно enum занимает 4 байта
            Type::Function { .. } => Some(0), // функции не имеют размера
            Type::TemplateInstance { .. } => None, // размер зависит от инстанцирования
            Type::Auto => None, // размер будет известен позже
            Type::Decltype(_) => None, // размер будет известен позже
            Type::Complex | Type::Imaginary => Some(8), // предполагаем двойной размер соответствующего типа
        }
    }

    pub fn alignment(&self) -> Option<usize> {
        match self {
            Type::Void => Some(1),
            Type::Bool | Type::Char => Some(1),
            Type::Short => Some(2),
            Type::Int | Type::Float => Some(4),
            Type::Long | Type::Double => Some(8),
            Type::Pointer(_) | Type::Reference(_) => Some(8),
            Type::Array(element_type, _) => element_type.alignment(),
            Type::Struct { fields, .. } => {
                let mut max_alignment = 1;
                for (_, field_type) in fields {
                    if let Some(alignment) = field_type.alignment() {
                        max_alignment = max_alignment.max(alignment);
                    }
                }
                Some(max_alignment)
            }
            Type::Union { fields, .. } => {
                let mut max_alignment = 1;
                for (_, field_type) in fields {
                    if let Some(alignment) = field_type.alignment() {
                        max_alignment = max_alignment.max(alignment);
                    }
                }
                Some(max_alignment)
            }
            _ => self.size_of(), // по умолчанию выравнивание равно размеру
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            Type::Bool
                | Type::Char
                | Type::Int
                | Type::Float
                | Type::Double
                | Type::Long
                | Type::Short
                | Type::Signed
                | Type::Unsigned
        )
    }

    pub fn is_scalar(&self) -> bool {
        self.is_arithmetic() || matches!(self, Type::Pointer(_) | Type::Enum { .. })
    }

    pub fn is_object(&self) -> bool {
        !matches!(self, Type::Function { .. } | Type::Void)
    }

    pub fn is_compound(&self) -> bool {
        matches!(
            self,
            Type::Pointer(_) | Type::Array(_, _) | Type::Struct { .. } | Type::Union { .. }
        )
    }

    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Type::Char | Type::Int | Type::Float | Type::Double | Type::Long | Type::Short
        )
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(
            self,
            Type::Bool | Type::Unsigned
        )
    }
}

#[derive(Debug)]
pub struct TypeRegistry {
    types: HashMap<String, Type>,
    next_id: u32,
}

impl TypeRegistry {
    pub fn new() -> Self {
        TypeRegistry {
            types: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn register_type(&mut self, name: String, ty: Type) -> Result<u32, String> {
        if self.types.contains_key(&name) {
            return Err(format!("Type '{}' already registered", name));
        }
        
        self.types.insert(name, ty);
        let id = self.next_id;
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn type_exists(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(Type::Int.size_of(), Some(4));
        assert_eq!(Type::Char.size_of(), Some(1));
        assert_eq!(Type::Double.size_of(), Some(8));
        
        let array_type = Type::Array(Box::new(Type::Int), 5);
        assert_eq!(array_type.size_of(), Some(20)); // 4 * 5
    }

    #[test]
    fn test_type_properties() {
        assert!(Type::Int.is_arithmetic());
        assert!(Type::Float.is_arithmetic());
        assert!(!Type::Void.is_arithmetic());
        
        assert!(Type::Int.is_scalar());
        assert!(Type::Pointer(Box::new(Type::Int)).is_scalar());
        assert!(!Type::Struct { name: "S".to_string(), fields: vec![] }.is_scalar());
    }

    #[test]
    fn test_type_registry() {
        let mut registry = TypeRegistry::new();
        
        let id = registry.register_type("MyInt".to_string(), Type::Int).unwrap();
        assert_eq!(id, 0);
        
        assert!(registry.get_type("MyInt").is_some());
        assert!(registry.type_exists("MyInt"));
        
        // Попытка зарегистрировать тот же тип снова должна завершиться ошибкой
        let result = registry.register_type("MyInt".to_string(), Type::Float);
        assert!(result.is_err());
    }
}