use std::fmt;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;
use std::string::ToString;

#[derive(Debug)]
pub struct BFProgram {
    filename: PathBuf,
<<<<<<< HEAD
    cells: Vec<InputInstruction>,
=======
    instructions: Vec<InputInstruction>,
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
}

impl BFProgram {
    pub fn new<T: AsRef<Path>>(a_path: T) -> BFProgram {
        BFProgram {
            filename: a_path.as_ref().to_path_buf(),
<<<<<<< HEAD
            cells: Vec::new(),
=======
            instructions: Vec::new(),
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
        }
    }

    pub fn filename(&self) -> &PathBuf {
        &self.filename
    }

<<<<<<< HEAD
    pub fn get_cell(&self, index: usize) -> &InputInstruction {
        &self.cells[index]
    }

    pub fn add_cell(&mut self, instruction: InputInstruction) {
        self.cells.push(instruction);
    }


    pub fn from_file<T: AsRef<Path>>(a_path: T) -> Result<BFProgram> {
=======
    pub fn instructions(&self) -> &Vec<InputInstruction> {
        &self.instructions
    }

    fn add_instruction(&mut self, instruction: InputInstruction) {
        self.instructions.push(instruction);
    }

    pub fn from_file<T: AsRef<Path>>(&self, a_path: T) -> Result<BFProgram> {
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
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
<<<<<<< HEAD
                        program.add_cell(instruction);
=======
                        program.add_instruction(instruction);
>>>>>>> 69366dabf2aa813158490e30dc39fbbd71101b2b
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
pub struct InputInstruction {
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

    fn get_command(&self) -> &BFCommand {
        &self.command
    }

    fn line_number(&self) -> usize {
        self.line_number
    }

    fn column_number(&self) -> usize {
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
