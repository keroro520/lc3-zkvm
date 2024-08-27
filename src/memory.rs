//! LC3 Memory Module
//!
//! This module implements the memory system for the LC3 (Little Computer 3) Zero-Knowledge Virtual Machine.
//!
//! ## Design
//! - The LC3 uses a 16-bit address space, allowing for 65,536 (2^16) memory locations.
//! - Each memory location stores a 16-bit word.
//! - The memory is implemented as a fixed-size array of 65,536 16-bit unsigned integers.
//! - Memory operations include reading, writing, and clearing.
//! - The module implements the `Index` and `IndexMut` traits for convenient array-like access.
//!
//! ## Usage
//! Create a new memory instance:
//! ```
//! use lc3_zkvm::memory::Memory;
//! let mut memory = Memory::new();
//!
//! // Read from and write to memory:
//! memory.write(0x3000, 0x1234);
//! let value = memory.read(0x3000);
//!
//! // Use array-like indexing:
//! memory[0x3000] = 0x5678;
//! let value = memory[0x3000];
//! ```

use std::ops::{Index, IndexMut};

pub const MEMORY_SIZE: usize = 65536; // 2^16, as LC3 uses 16-bit addressing

pub struct Memory {
    data: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }

    pub fn clear(&mut self) {
        self.data = [0; MEMORY_SIZE];
    }
}

impl Index<u16> for Memory {
    type Output = u16;

    fn index(&self, address: u16) -> &Self::Output {
        &self.data[address as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        &mut self.data[address as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_operations() {
        let mut mem = Memory::new();
        
        // Test write and read
        mem.write(0x3000, 0x1234);
        assert_eq!(mem.read(0x3000), 0x1234);

        // Test indexing
        mem[0x3001] = 0x5678;
        assert_eq!(mem[0x3001], 0x5678);

        // Test clear
        mem.clear();
        assert_eq!(mem[0x3000], 0);
        assert_eq!(mem[0x3001], 0);
    }
}