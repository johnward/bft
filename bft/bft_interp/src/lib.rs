use bft_types::BFProgram;
use bft_types::InputInstruction;
use std::io::Result;
use std::fmt;


#[derive(Debug)]
pub struct BFVirtualMachine {
    program: BFProgram,
    can_grow: bool,
    cell_idx: usize,
    tape_size: usize,
}

impl BFVirtualMachine {
    pub fn new(a_program: BFProgram, can_grow: bool, tape_size: usize) -> BFVirtualMachine {
        BFVirtualMachine {
            program: a_program,
            can_grow,
            cell_idx: 0,
            tape_size,
        }
    }

    pub fn get_current_cell(&self) -> &InputInstruction {
        self.program.get_cell(self.cell_idx)
    }

    pub fn next(&mut self) {
        self.cell_idx += 1;
    }

    pub fn grow_tape_size_to(&mut self, size: usize) -> Result<bool> {
        if self.can_grow && size > self.tape_size {
            self.tape_size = size;
            return Ok(true);
        }

        Ok(false)
    }
}

impl fmt::Display for BFVirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruct in self.program.cells().iter() {
            write!(f, " {}\n", instruct)?;
        }

        write!(f, "End of Program")
    }
}
