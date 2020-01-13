use bft_interp::BFVirtualMachine;
use bft_types::BFProgram;
use std::io::Cursor;
//use std::env::args;
use std::io::Write;
use std::result::Result;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (filename, cells_number) = cli::get_filename_and_cells();

    let mut buff = Cursor::new(vec![15, 23]);

    println!("Filename {}", filename);
    println!("Number of Cells: {}", cells_number);

    let program = BFProgram::new(filename);

    let mut virtual_machine: BFVirtualMachine<u8> = BFVirtualMachine::new(&program, false, 30000);

    //virtual_machine.interpret

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    let interp = virtual_machine.move_head_right();

    match interp {
        Ok(_s) => println!("It's OK!!!"),
        Err(_e) => println!("It's Not OK"),
    }

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    match virtual_machine.input(&mut buff) {
        Ok(_s) => println!("Written Correctly"),
        Err(_e) => println!("Write Error"),
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"hello world!!!!!!!!!\n")?;

    match virtual_machine.output(&mut handle) {
        // pass the borrow as mutable
        Ok(_) => println!("Read Correctly"),
        Err(_e) => println!("Read Error"),
    };

    let res = virtual_machine.interpret(&mut buff, &mut handle);

    match res {
        Ok(()) => println!("All OK"),
        Err(_e) => println!("Not OK"),
    }

    let interp = virtual_machine.move_head_right();

    match interp {
        Ok(_s) => println!("It's OK!!!"),
        Err(_e) => println!("It's Not OK"),
    }

    println!("Current Cell: {}", virtual_machine.get_current_cell());

    println!("---Virtual Machine---");
    println!("{}", virtual_machine);
    println!("---Virtual Machine---");

    let balanced = virtual_machine.has_matching_brackets().unwrap();

    println!("Has Matching Brackets: {}", balanced);

    Ok(())
}
