//! This is the BrainFuck interpreter
//!
//!
//!
//! This is a fully working Brainfuck interpretor
//! =============================================

use bft_types::BFCommand;
use bft_types::BFProgram;
use bft_types::InputInstruction;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::result::Result;
use std::vec::Vec;

/// This trait is for wrapping the u8 value
/// This trait covers add and subtract
///
/// ========================================
///
pub trait CellKind {
    fn wrapping_add(&mut self, num: u8);

    fn wrapping_sub(&mut self, num: u8);
}

/// Implementation for tje CellKind Trait
impl CellKind for u8 {
    fn wrapping_add(&mut self, num: u8) {
        if let Some(n) = self.checked_add(num) {
            *self = n;
        } else {
            *self = num - (u8::max_value() - *self);
        }
    }

    fn wrapping_sub(&mut self, num: u8) {
        if let Some(n) = self.checked_sub(num) {
            *self = n;
        } else {
            *self = u8::max_value() - (num - *self);
        }
    }
}

/// Error enum
#[derive(Debug, Clone, Copy)]
pub enum VMError {
    NoError(InputInstruction),
    InvalidHeadPosition(InputInstruction),
    TapeTooBig(InputInstruction),
    IOReadError(InputInstruction),
    IOWriteError(InputInstruction),
}

#[derive(Debug)]
pub struct BFVirtualMachine<'a, T> {
    program: &'a Vec<InputInstruction>,
    program_counter: usize,
    can_grow: bool,
    tape_pointer: usize,
    tape_size: usize,
    tape: Vec<u8>,
}

impl<'a, T> BFVirtualMachine<'a, T>
where
    T: Default + Clone + CellKind,
{
    pub fn new(
        a_program: &Vec<InputInstruction>,
        can_grow: bool,
        tape_size: usize,
    ) -> BFVirtualMachine<T> {
        let tape_size = if tape_size == 0 { 30000 } else { tape_size };
        let tape: Vec<u8> = std::iter::repeat(T::default()).take(tape_size).collect();
        BFVirtualMachine {
            program: a_program,
            program_counter: 0,
            can_grow,
            tape_pointer: 0,
            tape_size,
            tape,
        }
    }

    pub fn read(&self, reader: &mut impl Read) -> Result<usize, VMError> {
        let mut buffer = [0u8; 1];

        let instruct = self.program[self.program_counter];

        match reader.read(&mut buffer) {
            Ok(s) => {
                self.tape.insert(self.tape_pointer, buffer[0]);
                Ok(s)
            }
            Err(_) => Err(VMError::IOReadError(instruct)),
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<usize, VMError> {
        let mut buffer = [0u8; 1];
        let instruct = self.program[self.program_counter];
        buffer[0] = self.tape[self.tape_pointer];

        match writer.write(&buffer) {
            Ok(s) => Ok(s),
            Err(_) => Err(VMError::IOWriteError(instruct)),
        }
    }

    pub fn get_current_cell(&self) -> &InputInstruction {
        &self.program[self.tape_pointer]
    }

    pub fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.tape_pointer > 0 {
            self.tape_pointer -= 1;
            Ok(())
        } else {
            Err(VMError::InvalidHeadPosition(
                self.program[self.tape_pointer], // this needs changing
            ))
        }
    }

    pub fn move_head_right(&mut self) -> Result<(), VMError> {
        if self.tape_pointer < (self.tape_size - 1) {
            self.tape_pointer += 1;
            Ok(())
        } else {
            Err(VMError::InvalidHeadPosition(
                self.program[self.tape_pointer],
            ))
        }
    }

    pub fn wrapped_add(&mut self, num: u8) {
        self.tape[self.tape_pointer].wrapping_add(num);
    }

    pub fn wrapped_sub(&mut self, num: u8) {
        self.tape[self.tape_pointer].wrapping_sub(num);
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
    use super::CellKind;
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

    #[test]
    fn test_add_u8() {
        let mut aa: u8 = 25;
        assert_eq!(aa, 25);

        aa.wrapping_add(25);

        assert_eq!(aa, 50);
    }

    #[test]
    fn test_subtract_u8() {
        let mut aa: u8 = 25;
        assert_eq!(aa, 25);

        aa.wrapping_sub(25);

        assert_eq!(aa, 0);
    }

    #[test]
    fn test_add_wrap_u8() {
        let mut aa: u8 = 240;
        assert_eq!(aa, 240);

        aa.wrapping_add(50);
        println!("aa {}", aa);

        assert_eq!(aa, 35);
    }

    #[test]
    fn test_substract_wrap_u8() {
        let mut aa: u8 = 20;
        assert_eq!(aa, 20);

        aa.wrapping_sub(50);
        println!("aa {}", aa);

        assert_eq!(aa, 225);
    }
}
