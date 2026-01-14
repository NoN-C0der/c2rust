//! Шаблоны и дженерики

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TemplateParameter {
    pub name: String,
    pub constraint: Option<String>, // Ограничение на тип параметра
    pub default_type: Option<Type>, // Упрощенно используем Type из текущего крейта
}

#[derive(Debug, Clone)]
pub struct TemplateDeclaration {
    pub name: String,
    pub parameters: Vec<TemplateParameter>,
    pub body: String, // Упрощенное представление тела шаблона
}

impl TemplateDeclaration {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: Vec::new(),
            body: String::new(),
        }
    }

    pub fn add_parameter(mut self, param: TemplateParameter) -> Self {
        self.parameters.push(param);
        self
    }

    pub fn set_body(mut self, body: String) -> Self {
        self.body = body;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TemplateInstantiation {
    pub template_name: String,
    pub arguments: Vec<Type>, // Аргументы шаблона
    pub specialized_type: Type, // Результат инстанцирования
}

pub struct TemplateEnvironment {
    templates: HashMap<String, TemplateDeclaration>,
    instantiations: Vec<TemplateInstantiation>,
}

impl TemplateEnvironment {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            instantiations: Vec::new(),
        }
    }

    pub fn register_template(&mut self, template: TemplateDeclaration) {
        self.templates.insert(template.name.clone(), template);
    }

    pub fn instantiate_template(
        &mut self,
        name: String,
        args: Vec<Type>,
    ) -> Result<Type, String> {
        let template = self
            .templates
            .get(&name)
            .ok_or_else(|| format!("Template '{}' not found", name))?;

        if template.parameters.len() != args.len() {
            return Err(format!(
                "Template '{}' expects {} parameters, got {}",
                name,
                template.parameters.len(),
                args.len()
            ));
        }

        // В реальной реализации здесь была бы подстановка параметров
        // и генерация специализированного типа
        let instantiated_type = Type::Generic(format!("{}_inst", name));

        let instantiation = TemplateInstantiation {
            template_name: name,
            arguments: args,
            specialized_type: instantiated_type.clone(),
        };

        self.instantiations.push(instantiation);

        Ok(instantiated_type)
    }

    pub fn find_template(&self, name: &str) -> Option<&TemplateDeclaration> {
        self.templates.get(name)
    }
}

// Используем Type из нашего модуля
use crate::Type;