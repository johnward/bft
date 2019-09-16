use bft_interp::BFVirtualMachine;
use bft_types::BFProgram;
//use std::env::args;
use std::result::Result;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let filename = args().nth(1).ok_or("I need a filename")?;

    let (filename, cells_number) = cli::get_filename_and_cells();

    println!("Filename {}", filename);
    println!("Number of Cells: {}", cells_number);

    let program = BFProgram::from_file(filename).unwrap();

    let mut virtual_machine = BFVirtualMachine::new(program, false, 30000);

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    virtual_machine.next();

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    println!("---Virtual Machine---");
    println!("{}", virtual_machine);
    println!("---Virtual Machine---");

    Ok(())
}
