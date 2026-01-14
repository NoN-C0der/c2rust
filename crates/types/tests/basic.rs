use types::{Type, TypeInfo, Trait, TraitMethod};

#[test]
fn test_type_creation() {
    let int_type = Type::Int;
    assert!(int_type.is_primitive());
    
    let ptr_type = Type::Int.ptr();
    assert!(ptr_type.is_compound());
}

#[test]
fn test_type_info_creation() {
    let info = TypeInfo::new("test".to_string(), Type::Int, 4, 4);
    assert_eq!(info.name, "test");
}

#[test]
fn test_trait_creation() {
    let trait_ = Trait::new("Display".to_string())
        .add_method(TraitMethod {
            name: "display".to_string(),
            signature: "() -> String".to_string(),
            required: true,
        });
    
    assert_eq!(trait_.name, "Display");
    assert_eq!(trait_.methods.len(), 1);
}