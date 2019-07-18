extern crate crayne_lang;

use crayne_lang::vm::chunk::Chunk;
use crayne_lang::tools::disassembler::disassemble_chunk;

fn main() {
    let chunk = Chunk::test();
    println!("{}", disassemble_chunk(&chunk, "test"));
}