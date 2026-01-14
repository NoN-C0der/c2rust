//! Функции IR

use crate::{BasicBlock, instruction::Type};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
    pub basic_blocks: Vec<BasicBlock>,
    pub local_variables: HashMap<String, Type>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub r#type: Type,
}

impl Function {
    pub fn new(name: String, return_type: Type) -> Self {
        Self {
            name,
            return_type,
            parameters: Vec::new(),
            basic_blocks: Vec::new(),
            local_variables: HashMap::new(),
        }
    }

    pub fn add_parameter(mut self, param: Parameter) -> Self {
        self.parameters.push(param);
        self
    }

    pub fn add_basic_block(&mut self, block: BasicBlock) {
        self.basic_blocks.push(block);
    }

    pub fn add_local_variable(&mut self, name: String, r#type: Type) {
        self.local_variables.insert(name, r#type);
    }

    pub fn get_basic_block(&self, name: &str) -> Option<&BasicBlock> {
        self.basic_blocks.iter().find(|b| b.name == name)
    }

    pub fn get_basic_block_mut(&mut self, name: &str) -> Option<&mut BasicBlock> {
        self.basic_blocks.iter_mut().find(|b| b.name == name)
    }

    pub fn len(&self) -> usize {
        self.basic_blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.basic_blocks.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub global_variables: HashMap<String, Type>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            global_variables: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn add_global_variable(&mut self, name: String, r#type: Type) {
        self.global_variables.insert(name, r#type);
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.iter_mut().find(|f| f.name == name)
    }
}