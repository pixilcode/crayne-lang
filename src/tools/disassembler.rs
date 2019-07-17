use crate::vm::chunk::{OpCode, Chunk};

/// Disassemble a chunk into a human-readable
/// format
pub fn disassemble_chunk(chunk: Chunk, name: &str) -> String {
    format!("{}\n{}", chunk_header(name), chunk_body(&chunk, 0))
}

/// Create a chunk header
fn chunk_header(name: &str) -> String {
    format!("== {} ==", name)
}

/// Recursively create the body of a chunk
fn chunk_body(chunk: &Chunk, offset: usize) -> String {
    if offset >= chunk.size() {
        String::new()
    } else {
        let (result, next_offset) = disassemble_instruction(chunk, offset);
        format!("{}{}", result, chunk_body(chunk, next_offset))
    }
}

/// Disassemble an instruction into a
/// human-readable format and return the
/// text and the offset of the end of
/// the instruction
fn disassemble_instruction(chunk: &Chunk, offset: usize)
    -> (String, usize) {
    let (instruction, new_offset) =
        match OpCode::from(chunk.byte_at(offset)) {
            OpCode::Return => simple_instruction("OP_RETURN", offset),
            Constant => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::Invalid(code) => (
                format!("Unknown opcode: {}\n", code),
                offset + 1
            )
        };
    
    (
        format!("{:04} {}", offset, instruction),
        new_offset
    )
    
}

/// Create the text for a simple instruction
fn simple_instruction(text: &str, offset: usize) -> (String, usize) {
    (format!("{}\n", text), offset + 1)
}

/// Create a text for a constant
fn constant_instruction(text: &str, chunk: &Chunk, offset: usize) -> (String, usize) {
    let constant = chunk.byte_at(offset + 1);
    (format!("{:-16} {:4} '{}'\n", text, constant, chunk.const_val(constant)), offset + 2)
}

// Add tests here when a full VM has been implemented