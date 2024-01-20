use std::fs::File;
use std::io::{self, Read};

fn read_rom(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;
    return Ok(buf);
}

fn disassemble_rom(rom: &[u8]) {
    /* to be implemented: opcode parsing, disassembly logic */
    for opcode in rom.chunks(2) { // e/ instruction is 2 bytes long in chip8
        //parse & decode
        //print disassembled instruction
    }
}

fn main() {
    let filepath = "FIXME";
    match read_rom(filepath) {
        Ok(rom) => disassemble_rom(&rom),
        Err(e) => println!("Error reading ROM {}", e),
    }
}

