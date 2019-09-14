use bft_types::BFProgram;
use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;
use std::string::ToString;


#[derive(Debug)]
pub struct BFVirtualMachine<T> {
    program: &Vec<T>,
    can_grow: bool,
    cell_idx: usize,
    tape_size: usize,
}

impl<T> BFVirtualMachine<T> {
    pub fn new(a_program: &Vec<T>, can_grow: bool, tape_size: usize) -> BFVirtualMachine<T> {
        BFVirtualMachine {
            program: a_program,
            can_grow,
            cell_idx: 0,
            tape_size,
        }
    }
    pub fn current_cell(&self) -> &T {
        &self.program[self.cell_idx]
    }

    pub fn grow_tape_size_to(&mut self, size: usize) -> Result<bool> {
        if self.can_grow && size > self.tape_size {
            self.tape_size = size;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn load_tape(program: BFProgram) {
        // mmm
    }
}
