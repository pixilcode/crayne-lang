use std::fmt;

/// Represents a constant value in a
/// chunk
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Int(u32),
    DoesNotExist
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::DoesNotExist => write!(f, "Constant does not exist")
        }
    }
}

/// A vector that contains the constants
/// for a specific chunk
#[derive(PartialEq, Debug, Default)]
pub struct ConstantPool(Vec<Value>);

impl ConstantPool {
    /// Create a new constant pool
    pub fn new() -> Self {
        ConstantPool(vec![])
    }
    
    pub fn write(mut self, value: Value) -> Self {
        self.0.push(value);
        self
    }
    
    pub fn get_const(&self, index: usize) -> Value {
        self.0.get(index).cloned().unwrap_or(Value::DoesNotExist)
    }
}