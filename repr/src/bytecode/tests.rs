use std::mem;
use super::*;

#[test]
fn check_opcode_enum_size() {
    assert_eq!(mem::size_of::<OpCode>(), 1);
}

#[test]
fn check_opcode_byte_values() {
    assert_eq!(0, OpCode::Const as u8);
    assert_eq!(1, OpCode::VarAccess as u8);
    assert_eq!(2, OpCode::Not as u8);
    assert_eq!(3, OpCode::Sign as u8);
    assert_eq!(4, OpCode::Add as u8);
    assert_eq!(5, OpCode::Sub as u8);
    assert_eq!(6, OpCode::Mul as u8);
    assert_eq!(7, OpCode::Div as u8);
    assert_eq!(8, OpCode::Pow as u8);
    assert_eq!(9, OpCode::Mod as u8);
    assert_eq!(10, OpCode::Or as u8);
    assert_eq!(11, OpCode::And as u8);
    assert_eq!(12, OpCode::EqEq as u8);
    assert_eq!(13, OpCode::NotEq as u8);
    assert_eq!(14, OpCode::LessThan as u8);
    assert_eq!(15, OpCode::GreaterThan as u8);
    assert_eq!(16, OpCode::LessThanOrEq as u8);
    assert_eq!(17, OpCode::GreaterThanOrEq as u8);
    assert_eq!(18, OpCode::SkipnIfNZ as u8);
}

#[test]
fn test_iter_instr() {
    let bytecode = Bytecode {
        text: vec![],
        code: vec![
            // Const 3
            0, 0, 0, 0, 0, 0, 0, 8, 64,
            // Const 2
            0, 0, 0, 0, 0, 0, 0, 0, 64,
            // Mul
            6,
            // Const 1
            0, 0, 0, 0, 0, 0, 0, 240, 63,
            // Add
            4,
        ],
    };
    let mut iter = bytecode.instr_iter();
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Const,
        val: Value::from(3.0)
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Const,
        val: Value::from(2.0)
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Mul,
        val: NULL_VAL
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Const,
        val: Value::from(1.0)
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Add,
        val: NULL_VAL
    }));
}

#[test]
fn test_iter_instr_var_access() {
    let bytecode = Bytecode {
        text: vec![],
        code: vec![
            // VarAccess 0
            1, 0, 0, 0, 0, 0, 0, 0, 0,
            // Const 25.12
            0, 31, 133, 235, 81, 184, 30, 57, 64,
            // Mul
            6,
            // VarAccess 1
            1, 1, 0, 0, 0, 0, 0, 0, 0,
            // Add
            4,
            // SkipnIfNZ
            18, 1, 0, 0, 0, 0, 0, 0, 0,
        ]
    };
    let mut iter = bytecode.instr_iter();
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::VarAccess,
        val: Value::from(0)
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Const,
        val: Value::from(25.12)
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Mul,
        val: NULL_VAL
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::VarAccess,
        val: Value::from(1)
    }));
    assert_eq!(iter.next(), Some(Instruction {
        op: OpCode::Add,
        val: NULL_VAL
    }));
    let skip_inst = iter.next().unwrap();
    assert_eq!(skip_inst.op, OpCode::SkipnIfNZ);
    assert_eq!(skip_inst.val.raw_bits(), 1);
}
