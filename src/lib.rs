//! LC3 Virtual Machine
//!
//! This crate provides an implementation of the LC3 virtual machine, including modules for handling instructions, memory, opcodes, registers, and utility functions.
//!
//! # Modules
//!
//! - [`instruction`]: Contains functions to execute various LC3 instructions.
//! - [`memory`]: Manages the memory of the LC3 virtual machine.
//! - [`opcode`]: Defines the opcodes used by the LC3 virtual machine.
//! - [`register`]: Manages the registers of the LC3 virtual machine.
//! - [`utils`]: Provides utility functions used throughout the LC3 virtual machine.
//!
//! # Example
//!
//! ```rust
//! use lc3_zkvm::memory::Memory;
//! use lc3_zkvm::register::RegisterFile;
//! use lc3_zkvm::instruction::execute;
//!
//! let raw_instruction: u16 = 0x1234;
//! let mut registers = RegisterFile::new();
//! let mut memory = Memory::new();
//!
//! match execute(raw_instruction, &mut registers, &mut memory) {
//!     Ok(_) => println!("Instruction executed successfully"),
//!     Err(e) => println!("Instruction execution failed: {}", e),
//! }
//! ```

pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod register;
pub mod utils;

#[cfg(test)]
mod instruction_test;
