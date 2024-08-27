use lc3_zkvm::memory::Memory;
use lc3_zkvm::register::{Register, RegisterFile};
use lc3_zkvm::utils::{execute_program, load_obj_file};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: program <path_to_obj_file>".into());
    }

    let obj_file_path = &args[1];

    let mut memory = Memory::new();
    let mut registers = RegisterFile::new();

    // Load the LC3 object file
    let origin = load_obj_file(obj_file_path, &mut memory)?;

    // Set the PC to the program's origin
    registers.write(Register::PC, origin);

    // Execute the program
    match execute_program(&mut memory, &mut registers) {
        Ok(_) => println!("Program executed successfully"),
        Err(e) => println!("Error executing program: {}", e),
    }

    Ok(())
}
