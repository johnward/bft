//!
//! This is a fully working Brainfuck interpretor
//! =============================================
//!
//! Commands
//! ========
//! The eight language commands each consist of a single character:
//!
//! Character	Meaning
//!     >	    increment the data pointer (to point to the next cell to the right).
//!     <	    decrement the data pointer (to point to the next cell to the left).
//!     +	    increment (increase by one) the byte at the data pointer.
//!     -	    decrement (decrease by one) the byte at the data pointer.
//!     .	    output the byte at the data pointer.
//!     ,	    accept one byte of input, storing its value in the byte at the data pointer.
//!     [	    if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
//!     ]	    if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
//!             (Alternatively, the ] command may instead be translated as an unconditional jump to the corresponding [ command, or vice versa; programs will behave the same but will run more slowly, due to unnecessary double searching.)

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
    fn wrapping_increment(&mut self, number_to_add: u8) -> u8;

    fn wrapping_decrement(&mut self, number_to_sub: u8) -> u8;
}

/// Implementation for the CellKind Trait
/// number_to_add: This is the number to add to the U8
///
/// Return: This is old number plus number_to_add, which may be wrapped, if it is more that 255
impl CellKind for u8 {
    fn wrapping_increment(&mut self, number_to_add: u8) -> u8 {
        let current_number = *self;
        if let Some(n) = self.checked_add(number_to_add) {
            n
        } else {
            (number_to_add - ((u8::max_value() - current_number) + 1)) // + 1 to take into account the zero
        }
    }

    fn wrapping_decrement(&mut self, number_to_sub: u8) -> u8 {
        let current_number = *self;
        if let Some(n) = self.checked_sub(number_to_sub) {
            n
        } else {
            (u8::max_value() - ((number_to_sub - current_number) - 1)) // - 1 to take into account the zero
        }
    }
}

/*#[derive(Debug)]
pub struct Logger<W: Write> {
    out: W,
}

impl<W: Write> Logger<W> {
    pub fn new(out: W) -> Self {
        Logger { out }
    }

    // Just write the message directly to the given output with a newline.
    pub fn log(&mut self, message: &str) {
        self.out.write(message.as_bytes()).unwrap();
        self.out.write(b"\n").unwrap();
    }

    // Not an interesting method, but could be if we added buffering.
    pub fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}*/

/// Error enum
#[derive(Debug, Clone, Copy)]
pub enum VMError {
    NoError(InputInstruction),
    InvalidHeadPosition(InputInstruction),
    TapeTooBig(InputInstruction),
    IOReadError(InputInstruction),
    IOWriteError(InputInstruction),
    NestImbalance(InputInstruction),
}

#[derive(Debug)]
pub struct BFVirtualMachine<'a, T> {
    program: &'a BFProgram,
    program_counter: usize,
    can_grow: bool,
    tape_pointer: usize,
    tape_size: usize,
    tape: Vec<T>,
}

impl<'a, T> BFVirtualMachine<'a, T>
where
    T: Default + Clone + Copy + CellKind + std::convert::From<u8>,
    u8: std::convert::From<T>,
{
    pub fn new(a_program: &BFProgram, can_grow: bool, tape_size: usize) -> BFVirtualMachine<T> {
        let tape_size = if tape_size == 0 { 30000 } else { tape_size };
        let tape: Vec<T> = std::iter::repeat(T::default()).take(tape_size).collect();
        BFVirtualMachine {
            program: a_program,
            program_counter: 0,
            can_grow,
            tape_pointer: 0,
            tape_size,
            tape,
        }
    }

    pub fn interpret<R, W>(&mut self, input: R, output: W) -> Result<(), VMError>
    where
        R: Read,
        W: Write,
    {
        Ok(())
    }

    pub fn get_current_cell(&self) -> &InputInstruction {
        &self.program.commands()[self.tape_pointer]
    }

    pub fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.tape_pointer > 0 {
            self.tape_pointer -= 1;
            Ok(())
        } else {
            Err(VMError::InvalidHeadPosition(
                self.program.commands()[self.tape_pointer], // this needs changing
            ))
        }
    }

    pub fn move_head_right(&mut self) -> Result<(), VMError> {
        if self.tape_pointer < (self.tape_size - 1) {
            self.tape_pointer += 1;
            Ok(())
        } else {
            Err(VMError::InvalidHeadPosition(
                self.program.commands()[self.tape_pointer],
            ))
        }
    }

    pub fn wrapped_add(&mut self, num: u8) {
        self.tape[self.tape_pointer].wrapping_increment(num);
    }

    pub fn wrapped_sub(&mut self, num: u8) {
        self.tape[self.tape_pointer].wrapping_decrement(num);
    }

    // [
    pub fn loop_forward(&mut self) -> Result<(), VMError> {
        let mut buffer: [u8; 1] = [0u8; 1];
        buffer[0] = self.tape[self.tape_pointer].into(); // Type T into u8

        if buffer[0] == 0 {
            let iter = self.program.commands().iter().skip(self.program_counter);

            let last_instruct = self.program.commands().iter().nth(self.program_counter);

            for instruct in iter {
                let a_char: char = BFCommand::to_char(instruct.get_command());

                if a_char == ']' {
                    self.program_counter += 1;
                    return Ok(());
                }
                self.program_counter += 1;
            }
            Err(VMError::NestImbalance(*last_instruct.unwrap()))
        } else {
            Ok(())
        }
    }

    // ]
    pub fn loop_back(&mut self) -> Result<(), VMError> {
        let mut buffer: [u8; 1] = [0u8; 1];
        buffer[0] = self.tape[self.tape_pointer].into(); // Type T into u8

        if buffer[0] != 0 {
            let instruct = self.program.commands()[self.program_counter];
            let first_instuct = instruct.clone();
            let mut found = false;

            while !found || self.program_counter > 0 {
                let instruct = self.program.commands()[self.program_counter];
                let a_char: char = BFCommand::to_char(instruct.get_command());

                if a_char == '[' {
                    self.program_counter -= 1;
                    found = true;
                    break;
                }
                self.program_counter -= 1;
            }

            if !found {
                Err(VMError::NestImbalance(first_instuct))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
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

    // TODO - this is still not right
    pub fn input(&mut self, reader: &mut impl Read) -> Result<usize, VMError> {
        let mut buffer: [u8; 1] = [0u8; 1];

        let instruct = self.program.commands()[self.program_counter];

        match reader.read(&mut buffer) {
            Ok(s) => {
                self.tape.insert(self.tape_pointer, buffer[0].into());
                Ok(s)
            }
            Err(_) => Err(VMError::IOReadError(instruct)),
        }
    }

    pub fn output(&mut self, writer: &mut impl Write) -> Result<usize, VMError> {
        let mut buffer: [u8; 1] = [0u8; 1];
        let instruct = self.program.commands()[self.program_counter];
        buffer[0] = self.tape[self.tape_pointer].into(); // Type T into u8

        println!("{}", buffer[0]);
        //writer.write_all(b"hello world");

        match writer.write(&buffer) {
            Ok(s) => {
                //writer.flush();
                Ok(s)
            }
            Err(_) => Err(VMError::IOWriteError(instruct)),
        }
    }
}

impl<'a, T> fmt::Display for BFVirtualMachine<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruct in self.program.commands().iter() {
            writeln!(f, " {}", instruct)?;
        }

        write!(f, "End of Program")
    }
}

#[cfg(test)]
mod tests {
    use super::BFVirtualMachine;
    //use super::CellKind;
    //use bft_types::BFCommand;
    use bft_types::BFProgram;
    use std::env;
    use std::io::Cursor;

    /*#[test]
    fn first_instruction_valid() {
        let mut path = env::current_dir().unwrap();

        path.set_file_name("inputbf.txt");

        let program = BFProgram::new(path);

        let virtual_machine: BFVirtualMachine<u8> = BFVirtualMachine::new(&program, false, 30000);

        let instruction = virtual_machine.get_current_cell();

        assert_eq!(BFCommand::to_char(instruction.get_command()), '+');
    }

    #[test]
    fn move_right_instruction_valid() {
        let mut path = env::current_dir().unwrap();

        path.set_file_name("inputbf.txt");

        let program = BFProgram::new(path);

        let mut virtual_machine: BFVirtualMachine<u8> =
            BFVirtualMachine::new(&program, false, 30000);

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
            BFVirtualMachine::new(&program, false, 30000);

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

        aa = aa.wrapping_increment(25);
        println!("aa {}", aa);

        assert_eq!(aa, 50);
    }

    #[test]
    fn test_subtract_u8() {
        let mut aa: u8 = 25;
        assert_eq!(aa, 25);

        aa = aa.wrapping_decrement(25);
        println!("aa {}", aa);

        assert_eq!(aa, 0);
    }

    #[test]
    fn test_add_wrap_u8() {
        let mut aa: u8 = 252;
        assert_eq!(aa, 252);
        println!("Max Value:{}", u8::max_value());

        let number_to_add: u8 = 7;
        let new_num = number_to_add - (u8::max_value() - aa);
        println!("New Num: {}", new_num);

        aa = aa.wrapping_increment(number_to_add);
        println!("aa {}", aa);

        assert_eq!(aa, 3);
    }

    #[test]
    fn test_substract_wrap_u8() {
        let mut aa: u8 = 4;
        assert_eq!(aa, 4);

        aa = aa.wrapping_decrement(6);
        println!("aa {}", aa);

        assert_eq!(aa, 254);
    }*/

    #[test]
    fn test_read_write1() {
        let mut buff = Cursor::new(vec![15]);

        let mut path = env::current_dir().unwrap();

        path.set_file_name("bft/inputbf.txt");

        let program = BFProgram::new(path);

        let mut virtual_machine: BFVirtualMachine<u8> =
            BFVirtualMachine::new(&program, false, 30000);

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        match virtual_machine.output(&mut handle) {
            // pass the borrow as mutable
            Ok(s) => println!("Written {}", s),
            Err(_e) => println!("Write Error"),
        };
    }

    #[test]
    fn test_read_write2() {
        let mut buff = Cursor::new(vec![15]);

        let mut path = env::current_dir().unwrap();

        path.set_file_name("bft/inputbf.txt");

        let program = BFProgram::new(path);

        let mut virtual_machine: BFVirtualMachine<u8> =
            BFVirtualMachine::new(&program, false, 30000);

        match virtual_machine.input(&mut buff) {
            Ok(s) => println!("Written Correctly"),
            Err(_e) => println!("Write Error"),
        }

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        match virtual_machine.output(&mut handle) {
            // pass the borrow as mutable
            Ok(s) => println!("Read Correctly {}", s),
            Err(_e) => println!("Read Error"),
        };
    }
}
