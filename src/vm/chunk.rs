use crate::vm::value::Value;
use crate::vm::value::ConstantPool;

/// Represents the possible one-byte operation
/// codes (opcodes) that describe the instruction
/// that follows
#[derive(PartialEq, Debug)]
pub enum OpCode {
    Return,
    Constant,
    Invalid(u8)
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            invalid => OpCode::Invalid(invalid)
        }
    }
}

/// A series of bytecode instructions
#[derive(PartialEq, Debug)]
pub struct Chunk {
    code: Vec<u8>,
    constants: ConstantPool
}

impl Chunk {
    /// Create a new chunk
    fn new() -> Self {
        Chunk {
            code: vec![],
            constants: ConstantPool::new()
        }
    }
    
    /// Add a byte to the chunk
    fn write(mut self, byte: u8) -> Self {
        self.code.push(byte);
        self
    }
    
    /// Add a constant to the chunk
    fn add_constant(mut self, value: Value) -> Self {
        let constants = self.constants.write(value);
        Chunk {
            constants,
            ..self
        }
    }
    
    /// Return the byte at a specific offset
    /// 
    /// If the offset is outside the chunk, it
    /// will return the max `u8` value. This
    /// may cause unexpected behavior and should
    /// probably be changed later.
    pub fn byte_at(&self, offset: usize) -> u8 {
        *self.code.get(offset).unwrap_or(&u8::max_value())
    }
    
    /// Returns the constant denoted by the index
    /// 
    /// If the index is outside the const pool, it
    /// will return `Value::Invalid`
    pub fn const_val(&self, index: u8) -> Value {
        self.constants.get_const(index as usize)
    }
    
    /// Return the size of the chunk
    pub fn size(&self) -> usize {
        self.code.len()
    }
    
    /// A test chunk for manually testing/running
    /// that can be modified as needed. Should
    /// not be used for production code.
    pub fn test() -> Self {
        Chunk {
            code: vec![0, 1, 0],
            constants: ConstantPool::new().write(Value::Int(32))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn write_to_chunk() {
        let expected = Chunk {
            code: vec![1],
            constants: ConstantPool::new()
        };
        let actual = Chunk::new().write(1);
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn add_a_constant() {
        let expected = Chunk {
            code: vec![],
            constants: ConstantPool::new().write(Value::Int(1))
        };
        let actual = Chunk::new().add_constant(Value::Int(1));
        
        assert_eq!(expected, actual);
    }
}