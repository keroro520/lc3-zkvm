//! LC3 Opcode Module
//!
//! This module defines the opcodes for the LC3 (Little Computer 3) Zero-Knowledge Virtual Machine.
//! It includes enumerations for the opcodes and their associated functions.

/// Represents the LC3 opcodes
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// Branch
    /// Conditional branch based on condition codes (N, Z, P)
    /// Format: 0000 NZP PCoffset9
    OP_BR = 0x0,

    /// Add
    /// Add two values and store the result
    /// Format: 0001 DR SR1 000 SR2 (register mode)
    ///         0001 DR SR1 1 imm5 (immediate mode)
    OP_ADD = 0x1,

    /// Load
    /// Load a value from memory into a register
    /// Format: 0010 DR PCoffset9
    OP_LD = 0x2,

    /// Store
    /// Store a value from a register into memory
    /// Format: 0011 SR PCoffset9
    OP_ST = 0x3,

    /// Jump to Subroutine
    /// Jump to a subroutine and save the return address
    /// Format: 0100 1 PCoffset11 (JSR)
    ///         0100 000 BaseR 000000 (JSRR)
    OP_JSR = 0x4,

    /// Bitwise AND
    /// Perform bitwise AND operation
    /// Format: 0101 DR SR1 000 SR2 (register mode)
    ///         0101 DR SR1 1 imm5 (immediate mode)
    OP_AND = 0x5,

    /// Load Register
    /// Load a value from memory using a base register and offset
    /// Format: 0110 DR BaseR offset6
    OP_LDR = 0x6,

    /// Store Register
    /// Store a value to memory using a base register and offset
    /// Format: 0111 SR BaseR offset6
    OP_STR = 0x7,

    /// Return from Interrupt
    /// Unused in LC3
    /// Format: 1000 000000000000
    OP_RTI = 0x8,

    /// Bitwise NOT
    /// Perform bitwise NOT operation
    /// Format: 1001 DR SR 111111
    OP_NOT = 0x9,

    /// Load Indirect
    /// Load a value from memory using an address stored in memory
    /// Format: 1010 DR PCoffset9
    OP_LDI = 0xA,

    /// Store Indirect
    /// Store a value to memory using an address stored in memory
    /// Format: 1011 SR PCoffset9
    OP_STI = 0xB,

    /// Jump
    /// Jump to an address specified by a register
    /// Format: 1100 000 BaseR 000000
    OP_JMP = 0xC,

    /// Reserved
    /// Unused opcode
    /// Format: 1101 000000000000
    OP_RES = 0xD,

    /// Load Effective Address
    /// Load a memory address into a register
    /// Format: 1110 DR PCoffset9
    OP_LEA = 0xE,

    /// Execute Trap
    /// Execute a system call
    /// Format: 1111 0000 trapvect8
    OP_TRAP = 0xF,
}

impl Opcode {
    /// Convert a u16 to an Opcode
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Opcode::OP_BR),
            1 => Some(Opcode::OP_ADD),
            2 => Some(Opcode::OP_LD),
            3 => Some(Opcode::OP_ST),
            4 => Some(Opcode::OP_JSR),
            5 => Some(Opcode::OP_AND),
            6 => Some(Opcode::OP_LDR),
            7 => Some(Opcode::OP_STR),
            8 => Some(Opcode::OP_RTI),
            9 => Some(Opcode::OP_NOT),
            10 => Some(Opcode::OP_LDI),
            11 => Some(Opcode::OP_STI),
            12 => Some(Opcode::OP_JMP),
            13 => Some(Opcode::OP_RES),
            14 => Some(Opcode::OP_LEA),
            15 => Some(Opcode::OP_TRAP),
            _ => None,
        }
    }

    /// Convert an Opcode to a u16
    pub fn to_u16(self) -> u16 {
        self as u16
    }
}

/// Extract the opcode from a 16-bit instruction
pub fn extract_opcode(instruction: u16) -> Option<Opcode> {
    Opcode::from_u16(instruction >> 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_conversion() {
        assert_eq!(Opcode::from_u16(0), Some(Opcode::OP_BR));
        assert_eq!(Opcode::from_u16(15), Some(Opcode::OP_TRAP));
        assert_eq!(Opcode::from_u16(16), None);

        assert_eq!(Opcode::OP_BR.to_u16(), 0);
        assert_eq!(Opcode::OP_TRAP.to_u16(), 15);
    }

    #[test]
    fn test_extract_opcode() {
        assert_eq!(extract_opcode(0b0001_000_000_000_000), Some(Opcode::OP_ADD));
        assert_eq!(
            extract_opcode(0b1111_000_000_000_000),
            Some(Opcode::OP_TRAP)
        );
        assert_eq!(
            extract_opcode(0b1111_111_111_111_111),
            Some(Opcode::OP_TRAP)
        );
    }
}
