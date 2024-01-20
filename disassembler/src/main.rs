use std::fs::File;
use std::io::{self, Read};

const ROM_PATH: &str = "/home/alex/rchip8/stars.ch8";

fn read_rom(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;
    return Ok(buf);
}

fn disassemble_rom(rom: &[u8]) {
    // for now, just print what the byte is
    for (i, &byte) in rom.iter().enumerate() {
        println!("byte {}: {:02X}", i, byte);
    }
}

fn main() {
    let filepath = ROM_PATH;
    match read_rom(filepath) {
        Ok(rom) => disassemble_rom(&rom),
        Err(e) => println!("Error reading ROM {}", e),
    }
}

