use lc3_zkvm::memory::Memory;
use lc3_zkvm::register::{RegisterFile, Register};
use lc3_zkvm::utils::{load_obj_file, execute_program};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory = Memory::new();
    let mut registers = RegisterFile::new();

    // Load the LC3 object file
    let origin = load_obj_file("path/to/your/program.obj", &mut memory)?;

    // Set the PC to the program's origin
    registers.write(Register::PC, origin);

    // Execute the program with debug mode enabled
    match execute_program(&mut memory, &mut registers) {
        Ok(_) => println!("Program executed successfully"),
        Err(e) => println!("Error executing program: {}", e),
    }

    Ok(())
}