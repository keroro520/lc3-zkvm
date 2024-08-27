use crate::opcode::{extract_opcode, Opcode};
use crate::register::{RegisterFile, Register};
use crate::memory::Memory;
use crate::instruction::execute;


#[test]
fn test_add_instruction() {
    let mut registers = RegisterFile::new();
    let mut memory = Memory::new();

    // Set initial values in registers
    registers.write(Register::R0, 5);
    registers.write(Register::R1, 10);

    // Encode the ADD instruction (ADD R2, R0, R1)
    let instruction = 0b0001_010_000_000_001;
    // 0x1401;
    //0b0001_010_000_001_000;
    assert_eq!(extract_opcode(instruction), Some(Opcode::OP_ADD));

    // Execute the instruction
    execute(instruction, &mut registers, &mut memory).unwrap();

    // Check the result
    assert_eq!(registers.read(Register::R2), 15);
}