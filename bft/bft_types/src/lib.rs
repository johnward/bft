use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;
use std::string::ToString;

#[derive(Debug)]
struct BFProgram {
    filename: PathBuf,
    instructions: Vec<InputInstruction>,
}

impl BFProgram {
    pub fn new<T: AsRef<Path>>(a_path: T) -> BFProgram {
        BFProgram {
            filename: a_path.as_ref().to_path_buf(),
            instructions: Vec::new(),
        }
    }

    pub fn filename(&self) -> &PathBuf {
        &self.filename
    }

    pub fn instructions(&self) -> &Vec<InputInstruction> {
        &self.instructions
    }

    fn add_instruction(&mut self, instruction: InputInstruction) {
        self.instructions.push(instruction);
    }

    pub fn from_file<T: AsRef<Path>>(&self, a_path: T) -> Result<BFProgram> {
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
                        program.add_instruction(instruction);
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
enum BFCommand {
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
            '>' => Some(BFCommand::IncrementPointer(raw_command)),
            '<' => Some(BFCommand::DecrementPointer(raw_command)),
            '+' => Some(BFCommand::IncrementByte(raw_command)),
            '-' => Some(BFCommand::DecrementByte(raw_command)),
            '.' => Some(BFCommand::OutputByte(raw_command)),
            ',' => Some(BFCommand::InputByte(raw_command)),
            '[' => Some(BFCommand::IfZeroJumpForward(raw_command)),
            ']' => Some(BFCommand::IfNonZeroJumpBack(raw_command)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct InputInstruction {
    command: BFCommand,
    line_number: usize,
    column_number: usize,
}

impl InputInstruction {
    fn new(command: BFCommand, line_number: usize, column_number: usize) -> InputInstruction {
        InputInstruction {
            command,
            line_number,
            column_number,
        }
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
