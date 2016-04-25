use std::mem;
use super::{
    SIGN_BIT,
    QNAN,
    TAG_NULL,
    TAG_FALSE,
    TAG_TRUE
};

pub const NULL_VAL: Value  = Value(QNAN | TAG_NULL);
pub const FALSE_VAL: Value = Value(QNAN | TAG_FALSE);
pub const TRUE_VAL: Value  = Value(QNAN | TAG_TRUE);
//const UNDEFINED_VAL = Value(QNAN | TAG_UNDEFINED);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Value(u64);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ValueType {
    Number,
    Boolean,
    VarAccess
}

impl Value {

    pub fn get_type(self) -> ValueType {
        if self.0 & QNAN != QNAN {
            ValueType::Number
        } else if self == FALSE_VAL || self == TRUE_VAL {
            ValueType::Boolean
        } else {
            ValueType::VarAccess
        }
    }

    pub unsafe fn as_number_unchecked(self) -> f64 {
        mem::transmute(self.0)
    }

    pub unsafe fn as_var_access_unchecked(self) -> u64 {
        self.0 & !(QNAN | SIGN_BIT)
    }

    pub fn as_var_access(self) -> Result<u64, ()> {
        if self.0 & (QNAN | SIGN_BIT) == (QNAN | SIGN_BIT) {
            Ok(self.0 & !(QNAN | SIGN_BIT))
        } else {
            Err(())
        }
    }

    pub fn as_number(self) -> Result<f64, ()> {
        if self.0 & QNAN != QNAN {
            Ok(unsafe { mem::transmute(self.0) })
        } else {
            Err(())
        }
    }

    pub fn as_bool(self) -> bool {
        self.0 == TRUE_VAL.0
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value(unsafe { mem::transmute(value) })
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        if value { TRUE_VAL } else { FALSE_VAL }
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Value {
        if value > 1 << 51 { panic!("Value is too large!"); }
        Value(value | QNAN | SIGN_BIT)
    }
}

#[cfg(test)]
mod tests {

    use std::mem;
    use super::*;

    #[test]
    fn check_stack_value_size() {
        assert_eq!(mem::size_of::<Value>(), 8);
    }

    #[test]
    fn check_varaccess_type() {
        assert_eq!(
            Value::from(123).get_type(),
            ValueType::VarAccess
        );
        assert_eq!(
            123,
            Value::from(123).as_var_access().unwrap()
        );
    }

    #[test]
    fn check_bool_type() {
        assert_eq!(
            Value::from(true).get_type(),
            ValueType::Boolean
        );
        assert_eq!(
            true,
            Value::from(true).as_bool()
        );
        assert_eq!(
            false,
            Value::from(false).as_bool()
        );
    }
    #[test]
    fn check_number_type() {
        assert_eq!(
            Value::from(12.3e10).get_type(),
            ValueType::Number
        );
        assert_eq!(
            23.3f64,
            Value::from(23.3).as_number().unwrap()
        );
        assert!(
            Value::from(false).as_number().is_err()
        );
        assert!(
            Value::from(true).as_number().is_err()
        );
        assert!(
            Value::from(23).as_number().is_err()
        );
    }
}
