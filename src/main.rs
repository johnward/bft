use bft_interp::BFVirtualMachine;
use bft_types::BFProgram;
//use std::env::args;
use std::result::Result;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (filename, cells_number) = cli::get_filename_and_cells();

    println!("Filename {}", filename);
    println!("Number of Cells: {}", cells_number);

    let program = BFProgram::new(filename);

    let mut virtual_machine: BFVirtualMachine<u8> = BFVirtualMachine::new(program, false, 30000);

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    virtual_machine.next();

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    println!("---Virtual Machine---");
    println!("{}", virtual_machine);
    println!("---Virtual Machine---");

    let balanced = virtual_machine.has_matching_brackets().unwrap();

    println!("Has Matching Brackets: {}", balanced);

    Ok(())
}
