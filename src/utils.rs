use std::fs::File;
use std::io::{self, Read};
use crate::memory::Memory;
use crate::register::{RegisterFile, Register};
use crate::instruction::execute;
use crate::opcode::extract_opcode;

/// Load an LC3 object file into memory
pub fn load_obj_file(filename: &str, memory: &mut Memory) -> io::Result<u16> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut origin: u16 = 0x3000;
    let mut i = 0;

    // Read the origin
    if buffer.len() >= 2 {
        origin = u16::from_be_bytes([buffer[0], buffer[1]]);
        // origin = origin.swap_bytes();
        i = 2;
    }

    // Load program into memory
    let mut address = origin;
    while i < buffer.len() {
        if i + 1 < buffer.len() {
            let instruction = u16::from_be_bytes([buffer[i], buffer[i + 1]]);

            memory.write(address, instruction);

            // let opcode = extract_opcode(instruction);
            // println!("loading image, address: 0x{:04X}, opcode: {:?}, instruction: {:04X}", address, opcode.unwrap(), instruction   );

            address += 1;
            i += 2;
        } else {
            break;
        }
    }

    Ok(origin)
}

/// Execute the loaded program
pub fn execute_program(memory: &mut Memory, registers: &mut RegisterFile) -> Result<(), &'static str> {
    // println!("execute_program, PC: 0x{:04X}", registers.read(Register::PC));
    loop {
        let pc = registers.read(Register::PC);
        let raw_instruction = memory.read(pc);

        // Increment PC
        registers.write(Register::PC, pc.wrapping_add(1));

        if let Some(_opcode) = extract_opcode(raw_instruction) {
            // println!("execute_program, address: 0x{:04X}, opcode: {:?}", pc, opcode);

            match execute(raw_instruction, registers, memory) {
                Ok(_) => {},
                Err("HALT") => return Ok(()),
                Err(e) => return Err(e),
            }
        } else {
            return Err("Invalid instruction");
        }
    }
}