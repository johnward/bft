//! This is the documentaton for the crate itself
//! It can be mmany line of clde
//! 
//! Woop!
//! 
//! _Woop Again_
//! ==========

use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;
use std::string::ToString;

#[derive(Debug)]
pub struct BFProgram {
    filename: PathBuf,
    cells: Vec<InputInstruction>,
}

impl BFProgram {
    pub fn new<T: AsRef<Path>>(a_path: T) -> BFProgram {
        BFProgram {
            filename: a_path.as_ref().to_path_buf(),
            cells: Vec::new(),
        }
    }

    pub fn filename(&self) -> &PathBuf {
        &self.filename
    }

    pub fn get_cell(&self, index: usize) -> &InputInstruction {
        &self.cells[index]
    }

    pub fn cells(&self) -> &Vec<InputInstruction> {
        &self.cells
    }

    pub fn add_cell(&mut self, instruction: InputInstruction) {
        self.cells.push(instruction);
    }


    pub fn from_file<T: AsRef<Path>>(a_path: T) -> Result<BFProgram> {
        let content = std::fs::read_to_string(&a_path)?;

        let mut program = BFProgram::new(a_path);

        let mut line_num = 0;
        let mut col_num;

        for line in content.lines() {
            col_num = 0;
            for achar in line.chars() {
                match BFCommand::from_char(achar) {
                    Some(v) => {
                        let instruction = InputInstruction::new(v, line_num, col_num);
                        program.add_cell(instruction);
                    }
                    None => (),
                }
                col_num += 1;
            }
            line_num += 1;
        }

        Ok(program)
    }
}

#[derive(Debug)]
pub enum BFCommand {
    IncrementPointer(char),  //>
    DecrementPointer(char),  //<
    IncrementByte(char),     //+
    DecrementByte(char),     //-
    OutputByte(char),        //.
    InputByte(char),         //,
    IfZeroJumpForward(char), //[
    IfNonZeroJumpBack(char), //]
}

impl BFCommand {
    fn from_char(raw_command: char) -> Option<BFCommand> {
        match raw_command {
            '>' => {
                println!(">");
                Some(BFCommand::IncrementPointer(raw_command))
            },
            '<' => {
                println!("<");
                Some(BFCommand::DecrementPointer(raw_command))
            },
            '+' => {
                println!("+");
                Some(BFCommand::IncrementByte(raw_command))
            },
            '-' => {
                println!("-");
                Some(BFCommand::DecrementByte(raw_command))
            },
            '.' => {
                println!(".");
                Some(BFCommand::OutputByte(raw_command))
            },
            ',' => {
                println!(",");
                Some(BFCommand::InputByte(raw_command))
            },
            '[' => {
                println!("[");
                Some(BFCommand::IfZeroJumpForward(raw_command))
            },
            ']' => {
                println!("]");
                Some(BFCommand::IfNonZeroJumpBack(raw_command))
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct InputInstruction {
    command: BFCommand,
    line_number: usize,
    column_number: usize,
}

impl InputInstruction {
    pub fn new(command: BFCommand, line_number: usize, column_number: usize) -> InputInstruction {
        InputInstruction {
            command,
            line_number,
            column_number,
        }
    }

    pub fn get_command(&self) -> &BFCommand {
        &self.command
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn column_number(&self) -> usize {
        self.column_number
    }
}

impl fmt::Display for InputInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}] {}",
            self.line_number,
            self.column_number,
            get_raw_command(&self.command).unwrap()
        )
    }
}

fn get_raw_command(a_bfcommand: &BFCommand) -> Option<String> {
    match a_bfcommand {
        BFCommand::IncrementPointer('>') => Some(">".to_string()),
        BFCommand::DecrementPointer('<') => Some("<".to_string()),
        BFCommand::IncrementByte('+') => Some("+".to_string()),
        BFCommand::DecrementByte('-') => Some("-".to_string()),
        BFCommand::OutputByte('.') => Some(".".to_string()),
        BFCommand::InputByte(',') => Some(",".to_string()),
        BFCommand::IfZeroJumpForward('[') => Some("[".to_string()),
        BFCommand::IfNonZeroJumpBack(']') => Some("]".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::BFProgram;
    use super::BFCommand;
    use std::env;
    #[test]
    fn value_is_correct() {

        let mut path = env::current_dir().unwrap();
        path.set_file_name("inputbf.txt");

        let program = BFProgram::from_file(path).unwrap();

        let mut byte: u8 = 0b00000000;

        for cell in program.cells() {

            let a_bfcommand: &BFCommand = cell.get_command();

            match a_bfcommand {
                BFCommand::IncrementPointer('>')   => byte ^= 0b00000001,
                BFCommand::DecrementPointer('<')   => byte ^= 0b00000010,
                BFCommand::IncrementByte('+')      => byte ^= 0b00000100,
                BFCommand::DecrementByte('-')      => byte ^= 0b00001000,
                BFCommand::OutputByte('.')         => byte ^= 0b00010000,
                BFCommand::InputByte(',')          => byte ^= 0b00100000,
                BFCommand::IfZeroJumpForward('[')  => byte ^= 0b01000000,
                BFCommand::IfNonZeroJumpBack(']')  => byte ^= 0b10000000,
                _ => (),
                }
        }

        assert_eq!(byte, 0b11111111);
    }

    #[test]
    fn line_column_number_is_correct() {
        let mut path = env::current_dir().unwrap();
        path.set_file_name("inputbf.txt");

        let program = BFProgram::from_file(path).unwrap();

        let instruct1 = program.get_cell(0);
        assert_eq!(instruct1.line_number(), 7);
        assert_eq!(instruct1.column_number(), 7);

        let instruct2 = program.get_cell(7);
        assert_eq!(instruct2.line_number(), 9);
        assert_eq!(instruct2.column_number(), 11);
    }
}