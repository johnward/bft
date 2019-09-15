//! This is the types for the bf interpretor
//! 
//! 
//! 
//! _Woop Again_
//! ==========

use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;
use std::string::ToString;

#[derive(Debug)]
/// This is a struct containing: 
/// * The BrainFuck commands in a Vector
/// * The filename of which they were read from
pub struct BFProgram {
    filename: PathBuf,
    cells: Vec<InputInstruction>,
}

impl BFProgram {
    /// Create a new BFProgram
    /// 
    pub fn new<T: AsRef<Path>>(a_path: T) -> BFProgram {
        BFProgram {
            filename: a_path.as_ref().to_path_buf(),
            cells: Vec::new(),
        }
    }

    /// Return the filename 
    pub fn filename(&self) -> &PathBuf {
        &self.filename
    }

    /// Get the cell for a given index
    pub fn get_cell(&self, index: usize) -> &InputInstruction {
        &self.cells[index]
    }

    /// Get all of the bf cells
    pub fn cells(&self) -> &Vec<InputInstruction> {
        &self.cells
    }

    // Add a new cell
    pub fn add_cell(&mut self, instruction: InputInstruction) {
        self.cells.push(instruction);
    }

    /// Create a new BFProgram from a file
    /// 
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
                //println!(">");
                Some(BFCommand::IncrementPointer(raw_command))
            },
            '<' => {
                //println!("<");
                Some(BFCommand::DecrementPointer(raw_command))
            },
            '+' => {
                //println!("+");
                Some(BFCommand::IncrementByte(raw_command))
            },
            '-' => {
                //println!("-");
                Some(BFCommand::DecrementByte(raw_command))
            },
            '.' => {
                //println!(".");
                Some(BFCommand::OutputByte(raw_command))
            },
            ',' => {
                //println!(",");
                Some(BFCommand::InputByte(raw_command))
            },
            '[' => {
                //println!("[");
                Some(BFCommand::IfZeroJumpForward(raw_command))
            },
            ']' => {
                //println!("]");
                Some(BFCommand::IfNonZeroJumpBack(raw_command))
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
/// Struct to represent an Brainfuck command, 
/// line number 
/// column number
pub struct InputInstruction {
    command: BFCommand,
    line_number: usize,
    column_number: usize,
}

impl InputInstruction {
    /// Create an new InputInstruction, passing the bf command, along with the line and column number
    pub fn new(command: BFCommand, line_number: usize, column_number: usize) -> InputInstruction {
        InputInstruction {
            command,
            line_number,
            column_number,
        }
    }

    /// Return the bf command
    pub fn get_command(&self) -> &BFCommand {
        &self.command
    }

    /// Return the line number of the bf command
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Return the column number of the bf command
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

        let mut program_is_valid = true;

        for cell in program.cells() {

            let a_bfcommand: &BFCommand = cell.get_command();

            if program_is_valid {
                program_is_valid = match a_bfcommand {
                    BFCommand::IncrementPointer('>')   => true,
                    BFCommand::DecrementPointer('<')   => true,
                    BFCommand::IncrementByte('+')      => true,
                    BFCommand::DecrementByte('-')      => true,
                    BFCommand::OutputByte('.')         => true,
                    BFCommand::InputByte(',')          => true,
                    BFCommand::IfZeroJumpForward('[')  => true,
                    BFCommand::IfNonZeroJumpBack(']')  => true,
                    _ => false,
                }
            }
        }

        assert_eq!(program_is_valid, true);
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