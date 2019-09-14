use bft_types::BFProgram;
<<<<<<< HEAD
use bft_types::InputInstruction;
=======
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;
use std::string::ToString;


#[derive(Debug)]
<<<<<<< HEAD
pub struct BFVirtualMachine {
    program: BFProgram,
=======
pub struct BFVirtualMachine<T> {
    program: &Vec<T>,
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
    can_grow: bool,
    cell_idx: usize,
    tape_size: usize,
}

<<<<<<< HEAD
impl BFVirtualMachine {
    pub fn new(a_program: BFProgram, can_grow: bool, tape_size: usize) -> BFVirtualMachine {
=======
impl<T> BFVirtualMachine<T> {
    pub fn new(a_program: &Vec<T>, can_grow: bool, tape_size: usize) -> BFVirtualMachine<T> {
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
        BFVirtualMachine {
            program: a_program,
            can_grow,
            cell_idx: 0,
            tape_size,
        }
    }
<<<<<<< HEAD

    pub fn get_current_cell(&self) -> &InputInstruction {
        self.program.get_cell(self.cell_idx)
    }

    pub fn next(&mut self) {
        self.cell_idx += 1;
=======
    pub fn current_cell(&self) -> &T {
        &self.program[self.cell_idx]
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
    }

    pub fn grow_tape_size_to(&mut self, size: usize) -> Result<bool> {
        if self.can_grow && size > self.tape_size {
            self.tape_size = size;
            return Ok(true);
        }

        Ok(false)
    }
<<<<<<< HEAD
=======

    pub fn load_tape(program: BFProgram) {
        // mmm
    }
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
}
