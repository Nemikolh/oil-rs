use oil_repr::bytecode::Bytecode;
use oil_repr::bytecode::Value;
use super::VM;

#[test]
fn test_execution_correctness() {
    let mut vm = VM::new();
    let bytecode = Bytecode::new(vec![], vec![]);
    assert_eq!(vm.execute(&bytecode, ()).unwrap(), Value::from(12.0));
}
