use bft_types::BFProgram;
use bft_interp::BFVirtualMachine;
use std::env::args;
use std::fmt;
use std::option::Option;
use std::result::Result;


fn main() -> Result<(), Box<std::error::Error>> {
    let filename = args().nth(1).ok_or("I need a filename")?;

<<<<<<< HEAD
    let program = BFProgram::from_file(filename).unwrap();

    let virtual_machine = BFVirtualMachine::new(program, false, 30000);

    println!("Current Cell: {}", virtual_machine.get_current_cell());
=======
    let program = BFProgram::from_file(filename);

    

>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b

    Ok(())
}
