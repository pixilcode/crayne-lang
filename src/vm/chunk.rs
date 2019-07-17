/// Represents the possible one-byte operation
/// codes (opcodes) that describe the instruction
/// that follows
#[derive(PartialEq, Debug)]
enum OpCode {
    Return
}

/// A series of bytecode instructions
#[derive(PartialEq, Debug)]
struct Chunk {
    code: Vec<u8>
}

impl Chunk {
    /// Create a new chunk
    fn new() -> Self {
        Chunk {
            code: vec![]
        }
    }
    
    /// Add a byte to the chunk
    fn write(mut self, byte: u8) -> Self {
        self.code.push(byte);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn write_to_chunk() {
        let expected = Chunk {
            code: vec![1]
        };
        let actual = Chunk::new().write(1);
        
        assert_eq!(expected, actual);
    }
}