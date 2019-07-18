pub mod chunk;
pub mod value;

use chunk::{Chunk, OpCode};
use crate::debug;
use crate::tools::disassembler::disassemble_instruction;

/// The virtual machine
struct VM {
    chunk: Chunk
}

impl VM {
    /// Run the VM
    fn run(&self) -> VMResult {
        let mut ip = 0;
        let mut result = Ok(());
        
        loop {
            debug!(disassemble_instruction(&self.chunk, ip));
            let instruction = self.chunk.byte_at(ip);
            ip += 1;
            match OpCode::from(instruction) {
                OpCode::Return => break,
                OpCode::Constant => {
                    let constant = self.chunk.read_const(ip + 1);
                    // TODO Get rid of this
                    println!("{}", constant);
                },
                OpCode::Invalid(_) => {
                    result = Err(VMError::CompileError);
                    break;
                }
            }
        }
        
        result
    }
}

type VMResult = Result<(), VMError>;

enum VMError {
    CompileError,
    RuntimeError
}