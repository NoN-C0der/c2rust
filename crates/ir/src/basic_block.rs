//! Базовые блоки IR

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Instruction>, // Последняя инструкция, обычно переход
}

impl BasicBlock {
    pub fn new(name: String) -> Self {
        Self {
            name,
            instructions: Vec::new(),
            terminator: None,
        }
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        // Если инструкция является терминатором, сохраняем её отдельно
        match instr {
            Instruction::Jump { .. } |
            Instruction::Branch { .. } |
            Instruction::Return { .. } => {
                self.terminator = Some(instr);
            },
            _ => {
                self.instructions.push(instr);
            }
        }
    }

    pub fn add_instructions(&mut self, instrs: Vec<Instruction>) {
        for instr in instrs {
            self.add_instruction(instr);
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.terminator.is_some()
    }

    pub fn len(&self) -> usize {
        self.instructions.len() + if self.terminator.is_some() { 1 } else { 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty() && self.terminator.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub blocks: Vec<BasicBlock>,
    pub entry_block: String, // Имя входного блока
}

impl ControlFlowGraph {
    pub fn new(entry_block: String) -> Self {
        Self {
            blocks: Vec::new(),
            entry_block,
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    pub fn get_block(&self, name: &str) -> Option<&BasicBlock> {
        self.blocks.iter().find(|b| b.name == name)
    }

    pub fn get_block_mut(&mut self, name: &str) -> Option<&mut BasicBlock> {
        self.blocks.iter_mut().find(|b| b.name == name)
    }
}