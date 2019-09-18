use bft_types::BFProgram;
use bft_types::InputInstruction;
use std::fmt;
use std::io::Result;
use std::vec::Vec;

#[derive(Debug)]
pub struct BFVirtualMachine {
    program: BFProgram,
    can_grow: bool,
    cell_idx: usize,
    tape_size: usize,
    stack: Vec<String>,
}

impl BFVirtualMachine {
    pub fn new(a_program: BFProgram, can_grow: bool, tape_size: usize) -> BFVirtualMachine {
        BFVirtualMachine {
            program: a_program,
            can_grow,
            cell_idx: 0,
            tape_size,
            stack: Vec::new(),
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

    pub fn has_matching_brackets(&mut self) -> Result<bool> {
        let mut balanced = true;
        let mut local_stack: Vec<String> = Vec::new();

        for bfinstruction in self.program.cells().iter() {
            if !balanced {
                break;
            }

            let a_char: String = bfinstruction.get_raw_command().unwrap();
            if a_char == "[" {
                local_stack.push(a_char);
            } else if a_char == "]" {
                if local_stack.is_empty() {
                    balanced = false;
                } else {
                    local_stack.pop();
                }
            }
        }

        if balanced && local_stack.is_empty() {
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
