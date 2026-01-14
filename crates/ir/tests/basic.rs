use ir::{Function, Parameter, BasicBlock, Instruction, Constant, Operand, Type};

#[test]
fn test_function_creation() {
    let func = Function::new("main".to_string(), Type::Int)
        .add_parameter(Parameter {
            name: "x".to_string(),
            r#type: Type::Int,
        });
    
    assert_eq!(func.name, "main");
    assert_eq!(func.parameters.len(), 1);
}

#[test]
fn test_basic_block_creation() {
    let mut block = BasicBlock::new("entry".to_string());
    block.add_instruction(Instruction::new_const(
        "temp".to_string(),
        Constant::Integer(42),
    ));
    
    assert_eq!(block.len(), 1);
}