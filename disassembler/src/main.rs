use std::fs::File;
use std::io::{self, Read};
use std::env;

struct Opcode {
    pattern: &'static str,
    assembly: &'static str,
}

mod spec {
    use super::Opcode;
    pub fn get_opcode(hex: &str) -> Option<&'static Opcode> {
        OPCODES.iter().find(|&opcode| matches_pattern(hex, opcode.pattern))
    }

    pub const OPCODES: &[Opcode] = &[
        Opcode { pattern: "00E0", assembly: "CLS" },
        Opcode { pattern: "6xnn", assembly: "ADD Vx, nn"}
    ];

    fn matches_pattern(hex: &str, pattern: &str) -> bool { 
        if hex.len() != pattern.len() {
            return false
        }

        pattern.chars().zip(hex.chars()).all(|(p, h)| p == 'x' || p == h || p == 'n')
    }
}


fn read_rom(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    if buf.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty ROM"));
    }

    Ok(buf)
}

fn disassemble_rom(rom: &[u8]) {
    let mut i = 0;
    while i+1 < rom.len() {
        let instr = ((rom[i]) as u16) << 8 | rom[i+1] as u16; // bitwise magic
        // println!("instruction {}: {:04X}", i/2, instr);
        if let Some(instr_info) = spec::get_opcode(&format!("{:04X}", instr)) {
            println!("assembly: {}", disassemble_opcode(instr_info, instr));
        }
        i += 2;
    }
}

// todo understand this shamelessly copied fn
fn disassemble_opcode(opcode: &Opcode, instr: u16) -> String {
    let hex_str = format!("{:04X}", instr);
    let mut result = opcode.assembly.to_string();

    if let Some(idx) = opcode.pattern.find('x') {
        let register_value = (instr >> 8) & 0x0F; // Extract x value
        let register_str = format!("{:X}", register_value);
        result = result.replacen('x', &register_str, 1);
    }

    if let Some(idx) = opcode.pattern.find("nn") {
        let nn_value = instr & 0xFF; // Extract nn value
        let nn_str = format!("{:02X}", nn_value);
        result = result.replacen("nn", &nn_str, 1);
    }

    result + &format!(" (Hex: {})", hex_str)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom_path = args.get(1).unwrap_or_else(|| {
        eprintln!("Usage: {} /path/to/rom", args[0]);
        std::process::exit(1);
    });
    

    match read_rom(rom_path) {
        Ok(rom) => disassemble_rom(&rom),
        Err(e) => eprintln!("Error reading ROM {}", e),
    }
}