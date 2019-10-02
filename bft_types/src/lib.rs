//! This is the types for the bf interpretor
//!
//!
//!
//! _Woop Again_
//! ==========

use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
/// This is a struct containing:
/// * The BrainFuck commands in a Vector
/// * The filename of which they were read from
pub struct BFProgram {
    filename: PathBuf,
    commands: Vec<InputInstruction>,
}

impl BFProgram {
    /// Create a new BFProgram
    ///
    pub fn new<T: AsRef<Path>>(a_path: T) -> BFProgram {
        BFProgram {
            filename: a_path.as_ref().to_path_buf(),
            commands: BFProgram::from_file(a_path).expect("Unable to Read file"), // get content here.emulrate .collect etc
        }
    }

    /// Return the filename
    pub fn filename(&self) -> &PathBuf {
        &self.filename
    }

    /// Get the cell for a given index
    pub fn get_command(&self, index: usize) -> &InputInstruction {
        &self.commands[index]
    }

    /// Get all of the bf cells
    pub fn commands(&self) -> &Vec<InputInstruction> {
        &self.commands
    }

    // Add a new cell
    pub fn add_command(&mut self, instruction: InputInstruction) {
        self.commands.push(instruction);
    }

    /// Create a new BFProgram from a file
    ///
    pub fn from_file<T: AsRef<Path>>(a_path: T) -> Result<Vec<InputInstruction>> {
        let content = std::fs::read_to_string(&a_path)?;
        let mut commands = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col_num, achar) in line.chars().enumerate() {
                if let Some(v) = BFCommand::from_char(achar) {
                    let instruction = InputInstruction::new(v, line_num, col_num);
                    commands.push(instruction);
                }
            }
        }

        Ok(commands)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BFCommand {
    IncrementPointer,  //>
    DecrementPointer,  //<
    IncrementByte,     //+
    DecrementByte,     //-
    OutputByte,        //.
    InputByte,         //,
    IfZeroJumpForward, //[
    IfNonZeroJumpBack, //]
}

// Option because it could be none
impl BFCommand {
    pub fn from_char(raw_command: char) -> Option<BFCommand> {
        match raw_command {
            '>' => Some(BFCommand::IncrementPointer),
            '<' => Some(BFCommand::DecrementPointer),
            '+' => Some(BFCommand::IncrementByte),
            '-' => Some(BFCommand::DecrementByte),
            '.' => Some(BFCommand::OutputByte),
            ',' => Some(BFCommand::InputByte),
            '[' => Some(BFCommand::IfZeroJumpForward),
            ']' => Some(BFCommand::IfNonZeroJumpBack),
            _ => None,
        }
    }

    pub fn to_char(command: BFCommand) -> char {
        match command {
            BFCommand::IncrementPointer => '>',
            BFCommand::DecrementPointer => '<',
            BFCommand::IncrementByte => '+',
            BFCommand::DecrementByte => '-',
            BFCommand::OutputByte => '.',
            BFCommand::InputByte => ',',
            BFCommand::IfZeroJumpForward => '[',
            BFCommand::IfNonZeroJumpBack => ']',
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
    pub fn get_command(&self) -> BFCommand {
        self.command
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
            "[Line {}, Col {}] {}",
            self.line_number,
            self.column_number,
            BFCommand::to_char(self.command),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::BFCommand;
    use super::BFProgram;
    use std::env;
    #[test]
    fn value_is_correct() {
        let mut path = env::current_dir().unwrap();

        path.set_file_name("inputbf.txt");

        let program = BFProgram::new(path);

        let mut program_is_valid = true;

        for cell in program.commands() {
            let a_bfcommand: BFCommand = cell.get_command();

            if program_is_valid {
                program_is_valid = match a_bfcommand {
                    BFCommand::IncrementPointer => true,
                    BFCommand::DecrementPointer => true,
                    BFCommand::IncrementByte => true,
                    BFCommand::DecrementByte => true,
                    BFCommand::OutputByte => true,
                    BFCommand::InputByte => true,
                    BFCommand::IfZeroJumpForward => true,
                    BFCommand::IfNonZeroJumpBack => true,
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

        let program = BFProgram::new(path);

        let instruct1 = program.get_command(0);
        assert_eq!(instruct1.line_number(), 7);
        assert_eq!(instruct1.column_number(), 7);

        let instruct2 = program.get_command(7);
        assert_eq!(instruct2.line_number(), 9);
        assert_eq!(instruct2.column_number(), 11);
    }
}
