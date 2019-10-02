use bft_types::BFCommand;
use bft_types::BFProgram;
use bft_types::InputInstruction;
use std::fmt;
use std::result::Result;
use std::vec::Vec;

#[derive(Debug)]
pub struct BFVirtualMachine<T> {
    program: BFProgram,
    can_grow: bool,
    cell_idx: usize,
    tape_size: usize,
    tape: Vec<T>,
}

impl<T> BFVirtualMachine<T>
where
    T: Default + Clone,
{
    pub fn new(a_program: BFProgram, can_grow: bool, tape_size: usize) -> BFVirtualMachine<T> {
        let tape_size = if tape_size == 0 { 30000 } else { tape_size };
        let tape: Vec<T> = std::iter::repeat(T::default()).take(tape_size).collect();
        BFVirtualMachine {
            program: a_program,
            can_grow,
            cell_idx: 0,
            tape_size,
            tape,
        }
    }

    pub fn get_current_cell(&self) -> &InputInstruction {
        self.program.get_command(self.cell_idx)
    }

    pub fn next(&mut self) {
        self.cell_idx += 1;
    }

    pub fn has_matching_brackets(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut balanced = true;
        let mut local_stack: Vec<&InputInstruction> = Vec::new();

        for bfinstruction in self.program.commands().iter() {
            if !balanced {
                break;
            }

            let a_char: char = BFCommand::to_char(bfinstruction.get_command());
            if a_char == '[' {
                local_stack.push(bfinstruction);
            } else if a_char == ']' {
                if local_stack.is_empty() {
                    balanced = false;
                    println!("Bracket Error {}", bfinstruction);
                } else {
                    local_stack.pop();
                }
            }
        }

        if !balanced || !local_stack.is_empty() {
            if !local_stack.is_empty() {
                let instruct: &InputInstruction = local_stack.pop().unwrap();
                println!("Bracket Error {}", instruct);
            }

            return Ok(false);
        }

        Ok(true)
    }
}

impl<T> fmt::Display for BFVirtualMachine<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruct in self.program.commands().iter() {
            writeln!(f, " {}", instruct)?;
        }

        write!(f, "End of Program")
    }
}
