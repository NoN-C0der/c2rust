//! Определение типов

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Примитивные типы
    Int,
    Float,
    Double,
    Char,
    Bool,
    /// Пользовательские типы
    Struct(String),
    Enum(String),
    Union(String),
    /// Функциональные типы
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    /// Составные типы
    Pointer(Box<Type>),
    Reference(Box<Type>),
    Array(Box<Type>, usize),
    /// Параметризованные типы
    Generic(String),
    /// Пустой тип
    Void,
    /// Неизвестный тип
    Unknown,
}

impl Type {
    /// Создает указатель на тип
    pub fn ptr(self) -> Self {
        Type::Pointer(Box::new(self))
    }

    /// Создает ссылку на тип
    pub fn ref_(self) -> Self {
        Type::Reference(Box::new(self))
    }

    /// Создает массив типа с заданным размером
    pub fn array(self, size: usize) -> Self {
        Type::Array(Box::new(self), size)
    }

    /// Проверяет, является ли тип примитивным
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Int | Type::Float | Type::Double | Type::Char | Type::Bool | Type::Void
        )
    }

    /// Проверяет, является ли тип составным
    pub fn is_compound(&self) -> bool {
        matches!(self, Type::Pointer(_) | Type::Reference(_) | Type::Array(_, _))
    }

    /// Проверяет, является ли тип пользовательским
    pub fn is_user_defined(&self) -> bool {
        matches!(self, Type::Struct(_) | Type::Enum(_) | Type::Union(_))
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::Char => write!(f, "char"),
            Type::Bool => write!(f, "bool"),
            Type::Struct(name) => write!(f, "struct {}", name),
            Type::Enum(name) => write!(f, "enum {}", name),
            Type::Union(name) => write!(f, "union {}", name),
            Type::Function { params, return_type } => {
                let params_str = params
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn({}) -> {}", params_str, return_type)
            }
            Type::Pointer(inner) => write!(f, "*{}", inner),
            Type::Reference(inner) => write!(f, "&{}", inner),
            Type::Array(inner, size) => write!(f, "[{}; {}]", inner, size),
            Type::Generic(name) => write!(f, "{}", name),
            Type::Void => write!(f, "void"),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub r#type: Type,
    pub size: usize,
    pub alignment: usize,
}

impl TypeInfo {
    pub fn new(name: String, r#type: Type, size: usize, alignment: usize) -> Self {
        Self {
            name,
            r#type,
            size,
            alignment,
        }
    }
}