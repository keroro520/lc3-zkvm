use crate::memory::Memory;
use crate::opcode::{extract_opcode, Opcode};
use crate::register::{condition_flags, Register, RegisterFile};
use std::io::{self, Read, Write};

pub fn execute(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &mut Memory,
) -> Result<(), &'static str> {
    let opcode = extract_opcode(raw).ok_or("Unknown opcode")?;
    match opcode {
        Opcode::OP_ADD => execute_add(raw, registers),
        Opcode::OP_AND => execute_and(raw, registers),
        Opcode::OP_NOT => execute_not(raw, registers),
        Opcode::OP_BR => execute_br(raw, registers),
        Opcode::OP_JMP => execute_jmp(raw, registers),
        Opcode::OP_JSR => execute_jsr(raw, registers),
        Opcode::OP_LD => execute_ld(raw, registers, memory),
        Opcode::OP_LDI => execute_ldi(raw, registers, memory),
        Opcode::OP_LDR => execute_ldr(raw, registers, memory),
        Opcode::OP_LEA => execute_lea(raw, registers),
        Opcode::OP_ST => execute_st(raw, registers, memory),
        Opcode::OP_STI => execute_sti(raw, registers, memory),
        Opcode::OP_STR => execute_str(raw, registers, memory),
        Opcode::OP_TRAP => execute_trap(raw, registers, memory),
        Opcode::OP_RES => Err("Reserved opcode"),
        Opcode::OP_RTI => Err("RTI not implemented"),
    }
}

/// ADD - Add
///
/// Add two values and store the result in a register.
/// If bit [5] is 0, the second source operand is obtained from SR2.
/// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
fn execute_add(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
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
fn execute_and(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
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
fn execute_not(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
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
fn execute_br(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
    let pc = registers.read(Register::PC);
    let cond = registers.read(Register::COND);
    let n = (raw >> 11) & 0x1;
    let z = (raw >> 10) & 0x1;
    let p = (raw >> 9) & 0x1;

    if (n == 1 && cond & condition_flags::FL_NEG != 0)
        || (z == 1 && cond & condition_flags::FL_ZRO != 0)
        || (p == 1 && cond & condition_flags::FL_POS != 0)
    {
        let pc_offset = sign_extend(raw & 0x1FF, 9);
        registers.write(Register::PC, pc.wrapping_add(pc_offset));
    }

    Ok(())
}

/// JMP - Jump
///
/// Unconditional jump to the address specified by the contents of the base register.
/// Also used for RET (return from subroutine) when BaseR is R7.
fn execute_jmp(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
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
fn execute_jsr(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
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
fn execute_ld(raw: u16, registers: &mut RegisterFile, memory: &Memory) -> Result<(), &'static str> {
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
fn execute_ldi(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &Memory,
) -> Result<(), &'static str> {
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
fn execute_ldr(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &Memory,
) -> Result<(), &'static str> {
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
fn execute_lea(raw: u16, registers: &mut RegisterFile) -> Result<(), &'static str> {
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
fn execute_st(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &mut Memory,
) -> Result<(), &'static str> {
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
fn execute_sti(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &mut Memory,
) -> Result<(), &'static str> {
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
fn execute_str(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &mut Memory,
) -> Result<(), &'static str> {
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
fn execute_trap(
    raw: u16,
    registers: &mut RegisterFile,
    memory: &mut Memory,
) -> Result<(), &'static str> {
    let trapvect8 = raw & 0xFF;
    match trapvect8 {
        0x20 => trap_getc(registers),
        0x21 => trap_out(registers),
        0x22 => trap_puts(registers, memory),
        0x23 => trap_in(registers),
        0x24 => trap_putsp(registers, memory),
        0x25 => trap_halt(),
        _ => Err("Unknown TRAP vector"),
    }
}

fn trap_getc(registers: &mut RegisterFile) -> Result<(), &'static str> {
    let mut buffer = [0; 1];
    if io::stdin().read_exact(&mut buffer).is_ok() {
        registers.write(Register::R0, buffer[0] as u16);
        Ok(())
    } else {
        Err("Failed to read character")
    }
}

fn trap_out(registers: &mut RegisterFile) -> Result<(), &'static str> {
    let char = (registers.read(Register::R0) & 0xFF) as u8 as char;
    print!("{}", char);
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    Ok(())
}

fn trap_puts(registers: &mut RegisterFile, memory: &Memory) -> Result<(), &'static str> {
    let mut address = registers.read(Register::R0);
    loop {
        let char = (memory.read(address) & 0xFF) as u8 as char;
        if char == '\0' {
            break;
        }
        print!("{}", char);
        address += 1;
    }
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    Ok(())
}

fn trap_in(registers: &mut RegisterFile) -> Result<(), &'static str> {
    print!("Enter a character: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut buffer = [0; 1];
    if io::stdin().read_exact(&mut buffer).is_ok() {
        let char = buffer[0] as char;
        println!("{}", char);
        registers.write(Register::R0, buffer[0] as u16);
        Ok(())
    } else {
        Err("Failed to read character")
    }
}

fn trap_putsp(registers: &mut RegisterFile, memory: &Memory) -> Result<(), &'static str> {
    let mut address = registers.read(Register::R0);
    loop {
        let word = memory.read(address);
        let char1 = (word & 0xFF) as u8 as char;
        if char1 == '\0' {
            break;
        }
        print!("{}", char1);

        let char2 = ((word >> 8) & 0xFF) as u8 as char;
        if char2 != '\0' {
            print!("{}", char2);
        } else {
            break;
        }
        address += 1;
    }
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    Ok(())
}

fn trap_halt() -> Result<(), &'static str> {
    Err("HALT")
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
