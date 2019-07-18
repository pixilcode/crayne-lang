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
    constants: ConstantPool,
    lines: Vec<u32>
}

impl Chunk {
    /// Create a new chunk
    fn new() -> Self {
        Chunk {
            code: vec![],
            constants: ConstantPool::new(),
            lines: vec![]
        }
    }
    
    /// Add a byte to the chunk
    fn write(mut self, byte: u8, line: u32) -> Self {
        self.code.push(byte);
        self.lines.push(line);
        self
    }
    
    /// Add a constant to the chunk
    fn add_constant(self, value: Value) -> Self {
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
    /// may cause unexpected behavior and will
    /// probably be changed later.
    pub fn byte_at(&self, offset: usize) -> u8 {
        *self.code.get(offset).unwrap_or(&u8::max_value())
    }
    
    /// Return the constant denoted by the index
    /// 
    /// If the index is outside the const pool, it
    /// will return `Value::Invalid`
    pub fn const_val(&self, index: u8) -> Value {
        self.constants.get_const(index as usize)
    }
    
    /// Return the constant denoted by the value
    /// of a certain offset
    /// 
    /// This function is simply the compostition
    /// of `Chunk::byte_at` and `Chunk::read_const`
    pub fn read_const(&self, offset: usize) -> Value {
        self.const_val(self.byte_at(offset))
    }
    
    /// Returns the line of the code that the
    /// byte refers to
    /// 
    /// If the index is outside of the code,
    /// it will return `0`
    pub fn get_line(&self, index: usize) -> u32 {
        *self.lines.get(index).unwrap_or(&0)
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
            constants: ConstantPool::new().write(Value::Int(32)),
            lines: vec![1, 1, 1]
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
            constants: ConstantPool::new(),
            lines: vec![1]
        };
        let actual = Chunk::new().write(1, 1);
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn add_a_constant() {
        let expected = Chunk {
            code: vec![],
            constants: ConstantPool::new().write(Value::Int(1)),
            lines: vec![]
        };
        let actual = Chunk::new().add_constant(Value::Int(1));
        
        assert_eq!(expected, actual);
    }
}