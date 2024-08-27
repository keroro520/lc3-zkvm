//! LC3 Register Module
//!
//! This module defines the registers for the LC3 (Little Computer 3) Zero-Knowledge Virtual Machine.

/// Number of general-purpose registers in LC3
pub const R_COUNT: usize = 8;

/// Enum representing LC3 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// General Purpose Register 0
    R0 = 0,
    /// General Purpose Register 1
    R1 = 1,
    /// General Purpose Register 2
    R2 = 2,
    /// General Purpose Register 3
    R3 = 3,
    /// General Purpose Register 4
    R4 = 4,
    /// General Purpose Register 5
    R5 = 5,
    /// General Purpose Register 6
    R6 = 6,
    /// General Purpose Register 7
    R7 = 7,
    /// Program Counter
    PC = 8,
    /// Condition Flags
    COND = 9,
}

/// Condition Flags
pub mod condition_flags {
    /// Positive Flag
    pub const FL_POS: u16 = 1 << 0;
    /// Zero Flag
    pub const FL_ZRO: u16 = 1 << 1;
    /// Negative Flag
    pub const FL_NEG: u16 = 1 << 2;
}

/// Struct representing the LC3 register file
pub struct RegisterFile {
    registers: [u16; R_COUNT],
    pc: u16,
    cond: u16,
}

impl RegisterFile {
    /// Create a new RegisterFile with all registers initialized to 0
    pub fn new() -> Self {
        RegisterFile {
            registers: [0; R_COUNT],
            pc: 0,
            cond: 0,
        }
    }

    /// Read the value of a register
    pub fn read(&self, register: Register) -> u16 {
        match register {
            Register::R0 | Register::R1 | Register::R2 | Register::R3 |
            Register::R4 | Register::R5 | Register::R6 | Register::R7 => self.registers[register as usize],
            Register::PC => self.pc,
            Register::COND => self.cond,
        }
    }

    /// Write a value to a register
    pub fn write(&mut self, register: Register, value: u16) {
        match register {
            Register::R0 | Register::R1 | Register::R2 | Register::R3 |
            Register::R4 | Register::R5 | Register::R6 | Register::R7 => self.registers[register as usize] = value,
            Register::PC => self.pc = value,
            Register::COND => self.cond = value,
        }
    }

    /// Update condition flags based on a value
    pub fn update_flags(&mut self, value: u16) {
        if value == 0 {
            self.cond = condition_flags::FL_ZRO;
        } else if (value >> 15) == 1 {
            self.cond = condition_flags::FL_NEG;
        } else {
            self.cond = condition_flags::FL_POS;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_operations() {
        let mut reg_file = RegisterFile::new();

        // Test writing and reading general-purpose registers
        reg_file.write(Register::R0, 0x1234);
        assert_eq!(reg_file.read(Register::R0), 0x1234);

        reg_file.write(Register::R7, 0x7890);
        assert_eq!(reg_file.read(Register::R7), 0x7890);

        // Test writing and reading PC
        reg_file.write(Register::PC, 0x3000);
        assert_eq!(reg_file.read(Register::PC), 0x3000);

        // Test updating and reading condition flags
        reg_file.update_flags(0);
        assert_eq!(reg_file.read(Register::COND), condition_flags::FL_ZRO);

        reg_file.update_flags(1);
        assert_eq!(reg_file.read(Register::COND), condition_flags::FL_POS);

        reg_file.update_flags(0xFFFF);
        assert_eq!(reg_file.read(Register::COND), condition_flags::FL_NEG);
    }
}
