use crate::opcode::Opcode;
use crate::register::{Register, RegisterFile, condition_flags};
use crate::memory::Memory;

pub struct Instruction {
    pub opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode }
    }

    pub fn execute(&self, raw: u16, registers: &mut RegisterFile, memory: &mut Memory) -> Result<(), &'static str> {
        match self.opcode {
            Opcode::OP_ADD => self.execute_add(raw, registers),
            Opcode::OP_AND => self.execute_and(raw, registers),
            Opcode::OP_NOT => self.execute_not(raw, registers),
            Opcode::OP_BR => self.execute_br(raw, registers),
            Opcode::OP_JMP => self.execute_jmp(raw, registers),
            Opcode::OP_JSR => self.execute_jsr(raw, registers),
            Opcode::OP_LD => self.execute_ld(raw, registers, memory),
            Opcode::OP_LDI => self.execute_ldi(raw, registers, memory),
            Opcode::OP_LDR => self.execute_ldr(raw, registers, memory),
            Opcode::OP_LEA => self.execute_lea(raw, registers),
            Opcode::OP_ST => self.execute_st(raw, registers, memory),
            Opcode::OP_STI => self.execute_sti(raw, registers, memory),
            Opcode::OP_STR => self.execute_str(raw, registers, memory),
            Opcode::OP_TRAP => self.execute_trap(raw, registers, memory),
            Opcode::OP_RES => Err("Reserved opcode"),
            Opcode::OP_RTI => Err("RTI not implemented"),
        }
    }

    /// ADD - Add
    /// 
    /// Add two values and store the result in a register.
    /// If bit [5] is 0, the second source operand is obtained from SR2.
    /// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
    fn execute_add(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let sr1 = (raw >> 6) & 0x7;
        let mode = (raw >> 5) & 0x1;

        let val1 = registers.read(Register::from(sr1));
        let val2 = if mode == 0 {
            let sr2 = raw & 0x7;
            registers.read(Register::from(sr2))
        } else {
            sign_extend(raw & 0x1F, 5)
        };

        let result = val1.wrapping_add(val2);
        registers.write(Register::from(dr), result);
        registers.update_flags(result);

        Ok(())
    }

    /// AND - Bitwise AND
    /// 
    /// Perform bitwise AND on two values and store the result in a register.
    /// If bit [5] is 0, the second source operand is obtained from SR2.
    /// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
    fn execute_and(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let sr1 = (raw >> 6) & 0x7;
        let mode = (raw >> 5) & 0x1;

        let val1 = registers.read(Register::from(sr1));
        let val2 = if mode == 0 {
            let sr2 = raw & 0x7;
            registers.read(Register::from(sr2))
        } else {
            sign_extend(raw & 0x1F, 5)
        };

        let result = val1 & val2;
        registers.write(Register::from(dr), result);
        registers.update_flags(result);

        Ok(())
    }

    /// NOT - Bitwise NOT
    /// 
    /// Perform bitwise NOT on a value and store the result in a register.
    fn execute_not(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let sr = (raw >> 6) & 0x7;

        let val = registers.read(Register::from(sr));
        let result = !val;
        registers.write(Register::from(dr), result);
        registers.update_flags(result);

        Ok(())
    }

    /// BR - Branch
    /// 
    /// Conditional branch based on condition codes (N, Z, P).
    /// If (n AND N) OR (z AND Z) OR (p AND P) is true, the program branches to the address specified by adding the sign-extended PCoffset9 field to the incremented PC.
    fn execute_br(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let pc = registers.read(Register::PC);
        let cond = registers.read(Register::COND);
        let n = (raw >> 11) & 0x1;
        let z = (raw >> 10) & 0x1;
        let p = (raw >> 9) & 0x1;

        if (n == 1 && cond & condition_flags::FL_NEG != 0) ||
           (z == 1 && cond & condition_flags::FL_ZRO != 0) ||
           (p == 1 && cond & condition_flags::FL_POS != 0) {
            let pc_offset = sign_extend(raw & 0x1FF, 9);
            registers.write(Register::PC, pc.wrapping_add(pc_offset));
        }

        Ok(())
    }

    /// JMP - Jump
    /// 
    /// Unconditional jump to the address specified by the contents of the base register.
    /// Also used for RET (return from subroutine) when BaseR is R7.
    fn execute_jmp(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let base_r = (raw >> 6) & 0x7;
        let base = registers.read(Register::from(base_r));
        registers.write(Register::PC, base);
        Ok(())
    }

    /// JSR - Jump to Subroutine
    /// 
    /// Jump to a subroutine, saving the return address in R7.
    /// If bit [11] is 0, the PC is loaded with the contents of the base register (JSRR).
    /// If bit [11] is 1, the PC is loaded with the address specified by adding the sign-extended PCoffset11 field to the incremented PC (JSR).
    fn execute_jsr(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let pc = registers.read(Register::PC);
        registers.write(Register::R7, pc);

        if (raw >> 11) & 0x1 == 1 {
            let pc_offset = sign_extend(raw & 0x7FF, 11);
            registers.write(Register::PC, pc.wrapping_add(pc_offset));
        } else {
            let base_r = (raw >> 6) & 0x7;
            let base = registers.read(Register::from(base_r));
            registers.write(Register::PC, base);
        }

        Ok(())
    }

    /// LD - Load
    /// 
    /// Load a value from memory into a register.
    /// The address is calculated by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    fn execute_ld(&self, raw: u16, registers: &mut RegisterFile, memory: &Memory) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let pc = registers.read(Register::PC);
        let pc_offset = sign_extend(raw & 0x1FF, 9);
        let address = pc.wrapping_add(pc_offset);
        let val = memory.read(address);
        registers.write(Register::from(dr), val);
        registers.update_flags(val);
        Ok(())
    }

    /// LDI - Load Indirect
    /// 
    /// Load a value from memory into a register using an indirect address.
    /// The address of the address is calculated by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    fn execute_ldi(&self, raw: u16, registers: &mut RegisterFile, memory: &Memory) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let pc = registers.read(Register::PC);
        let pc_offset = sign_extend(raw & 0x1FF, 9);
        let address = pc.wrapping_add(pc_offset);
        let indirect_address = memory.read(address);
        let val = memory.read(indirect_address);
        registers.write(Register::from(dr), val);
        registers.update_flags(val);
        Ok(())
    }

    /// LDR - Load Register
    /// 
    /// Load a value from memory into a register.
    /// The address is calculated by sign-extending bits [5:0] to 16 bits and adding this value to the contents of the base register.
    fn execute_ldr(&self, raw: u16, registers: &mut RegisterFile, memory: &Memory) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let base_r = (raw >> 6) & 0x7;
        let offset = sign_extend(raw & 0x3F, 6);
        let base = registers.read(Register::from(base_r));
        let address = base.wrapping_add(offset);
        let val = memory.read(address);
        registers.write(Register::from(dr), val);
        registers.update_flags(val);
        Ok(())
    }

    /// LEA - Load Effective Address
    /// 
    /// Load a register with an effective address.
    /// The address is calculated by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    fn execute_lea(&self, raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
        let dr = (raw >> 9) & 0x7;
        let pc = registers.read(Register::PC);
        let pc_offset = sign_extend(raw & 0x1FF, 9);
        let address = pc.wrapping_add(pc_offset);
        registers.write(Register::from(dr), address);
        registers.update_flags(address);
        Ok(())
    }

    /// ST - Store
    /// 
    /// Store a value from a register into memory.
    /// The address is calculated by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    fn execute_st(&self, raw: u16, registers: &mut RegisterFile, memory: &mut Memory) -> Result<(), &'static str> {
        let sr = (raw >> 9) & 0x7;
        let pc = registers.read(Register::PC);
        let pc_offset = sign_extend(raw & 0x1FF, 9);
        let address = pc.wrapping_add(pc_offset);
        let val = registers.read(Register::from(sr));
        memory.write(address, val);
        Ok(())
    }

    /// STI - Store Indirect
    /// 
    /// Store a value from a register into memory using an indirect address.
    /// The address of the address is calculated by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC.
    fn execute_sti(&self, raw: u16, registers: &mut RegisterFile, memory: &mut Memory) -> Result<(), &'static str> {
        let sr = (raw >> 9) & 0x7;
        let pc = registers.read(Register::PC);
        let pc_offset = sign_extend(raw & 0x1FF, 9);
        let address = pc.wrapping_add(pc_offset);
        let indirect_address = memory.read(address);
        let val = registers.read(Register::from(sr));
        memory.write(indirect_address, val);
        Ok(())
    }

    /// STR - Store Register
    /// 
    /// Store a value from a register into memory.
    /// The address is calculated by sign-extending bits [5:0] to 16 bits and adding this value to the contents of the base register.
    fn execute_str(&self, raw: u16, registers: &mut RegisterFile, memory: &mut Memory) -> Result<(), &'static str> {
        let sr = (raw >> 9) & 0x7;
        let base_r = (raw >> 6) & 0x7;
        let offset = sign_extend(raw & 0x3F, 6);
        let base = registers.read(Register::from(base_r));
        let address = base.wrapping_add(offset);
        let val = registers.read(Register::from(sr));
        memory.write(address, val);
        Ok(())
    }

    /// TRAP - System Call
    /// 
    /// Perform a system call specified by the trap vector.
    /// The trap vector is specified in bits [7:0] of the instruction.
    fn execute_trap(&self, raw: u16, _registers: &mut RegisterFile, _memory: &mut Memory) -> Result<(), &'static str> {
        // TRAP implementation depends on your specific ZKVM requirements
        // This is a placeholder implementation
        let trapvect8 = raw & 0xFF;
        match trapvect8 {
            0x20 => {
                // GETC: Read a single character from the keyboard
                // Not implemented in this example
                Err("GETC not implemented")
            }
            0x21 => {
                // OUT: Write a character to the console
                // Not implemented in this example
                Err("OUT not implemented")
            }
            0x22 => {
                // PUTS: Write a string to the console
                // Not implemented in this example
                Err("PUTS not implemented")
            }
            0x23 => {
                // IN: Print a prompt on the screen and read a character from the keyboard
                // Not implemented in this example
                Err("IN not implemented")
            }
            0x24 => {
                // PUTSP: Write a string of characters to the console
                // Not implemented in this example
                Err("PUTSP not implemented")
            }
            0x25 => {
                // HALT: Halt execution and print a message on the console
                Err("HALT")
            }
            _ => Err("Unknown TRAP vector"),
        }
    }
}

// Helper function: Sign extend a value with a given bit count
fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if ((x >> (bit_count - 1)) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}
// Helper function: Convert u16 to Register
impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            _ => panic!("Invalid register number"),
        }
    }
}