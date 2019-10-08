use bft_types::BFCommand;
use bft_types::BFProgram;
use bft_types::InputInstruction;
use std::fmt;
use std::result::Result;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum VMError {
    InvalidHeadPosition(InputInstruction),
    TapeTooBig(InputInstruction),
}

#[derive(Debug)]
pub struct BFVirtualMachine<'a, T> {
    program: &'a Vec<InputInstruction>,
    can_grow: bool,
    program_counter: usize,
    tape_size: usize,
    tape: Vec<T>,
}

impl<'a, T> BFVirtualMachine<'a, T>
where
    T: Default + Clone,
{
    pub fn new(
        a_program: &Vec<InputInstruction>,
        can_grow: bool,
        tape_size: usize,
    ) -> BFVirtualMachine<T> {
        let tape_size = if tape_size == 0 { 30000 } else { tape_size };
        let tape: Vec<T> = std::iter::repeat(T::default()).take(tape_size).collect();
        BFVirtualMachine {
            program: a_program,
            can_grow,
            program_counter: 0,
            tape_size,
            tape,
        }
    }

    pub fn get_current_cell(&self) -> &InputInstruction {
        &self.program[self.program_counter]
    }

    pub fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.program_counter > 0 {
            self.program_counter -= 1;
            Ok(())
        } else {
            Err(VMError::InvalidHeadPosition(
                self.program[self.program_counter],
            ))
        }
    }

    pub fn move_head_right(&mut self) -> Result<(), VMError> {
        if self.program_counter < (self.tape_size - 1) {
            self.program_counter += 1;
            Ok(())
        } else {
            Err(VMError::InvalidHeadPosition(
                self.program[self.program_counter],
            ))
        }
    }

    pub fn has_matching_brackets(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut balanced = true;
        let mut local_stack: Vec<&InputInstruction> = Vec::new();

        for bfinstruction in self.program.iter() {
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

impl<'a, T> fmt::Display for BFVirtualMachine<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruct in self.program.iter() {
            writeln!(f, " {}", instruct)?;
        }

        write!(f, "End of Program")
    }
}

#[cfg(test)]
mod tests {
    use super::BFProgram;
    use super::BFVirtualMachine;
    use bft_types::BFCommand;
    use std::env;

    #[test]
    fn first_instruction_valid() {
        let mut path = env::current_dir().unwrap();

        path.set_file_name("inputbf.txt");

        let program = BFProgram::new(path);

        let virtual_machine: BFVirtualMachine<u8> =
            BFVirtualMachine::new(program.commands(), false, 30000);

        let instruction = virtual_machine.get_current_cell();

        assert_eq!(BFCommand::to_char(instruction.get_command()), '+');
    }

    #[test]
    fn move_right_instruction_valid() {
        let mut path = env::current_dir().unwrap();

        path.set_file_name("inputbf.txt");

        let program = BFProgram::new(path);

        let mut virtual_machine: BFVirtualMachine<u8> =
            BFVirtualMachine::new(program.commands(), false, 30000);

        for num in 1..3 {
            let result = virtual_machine.move_head_right();

            match result {
                Ok(()) => (),
                Err(_e) => (),
            }

            let instruction = virtual_machine.get_current_cell();
            println!("Instrc: {}", BFCommand::to_char(instruction.get_command()));

            println!("Right: {}", num);
        }

        let instruction = virtual_machine.get_current_cell();

        assert_eq!(BFCommand::to_char(instruction.get_command()), '-');
    }

    #[test]
    fn move_left_instruction_valid() {
        let mut path = env::current_dir().unwrap();

        path.set_file_name("inputbf.txt");

        let program = BFProgram::new(path);

        let mut virtual_machine: BFVirtualMachine<u8> =
            BFVirtualMachine::new(program.commands(), false, 30000);

        for num in 1..3 {
            let result = virtual_machine.move_head_right();

            match result {
                Ok(()) => (),
                Err(_e) => (),
            }

            let instruction = virtual_machine.get_current_cell();
            println!("Instrc: {}", BFCommand::to_char(instruction.get_command()));

            println!("Right: {}", num);
        }

        let result = virtual_machine.move_head_left();

        match result {
            Ok(()) => (),
            Err(_e) => (),
        }

        let instruction = virtual_machine.get_current_cell();

        assert_eq!(BFCommand::to_char(instruction.get_command()), '[');
    }
}
