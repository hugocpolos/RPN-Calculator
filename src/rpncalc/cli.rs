use std::io;
use std::io::Write;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CliOperation {
    Push(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Clear,
    List,
    Quit,
    Unknown,
    Help,
    Empty,
}

pub struct CliCmd {
    pub oper: CliOperation,
}

impl CliCmd {
    fn from_raw_command(s: String) -> CliCmd {
        let tokenized_command = s.split_whitespace().collect::<Vec<_>>();

        if tokenized_command.is_empty() {
            return CliCmd::new_empty_command();
        }

        if tokenized_command[0].parse::<f64>().is_ok() {
            return CliCmd::new_push_command(f64::from_str(tokenized_command[0]).unwrap());
        }

        match tokenized_command[0].to_lowercase().as_str() {
            "+" | "a" | "add" => CliCmd::new_add_command(),
            "-" | "s" | "sub" => CliCmd::new_subtract_command(),
            "*" | "x" | "mul" => CliCmd::new_multiply_command(),
            "/" | "d" | "div" => CliCmd::new_divide_command(),
            "c" | "clear" => CliCmd::new_clear_command(),
            "p" | "print" => CliCmd::new_list_command(),
            "h" | "help" => CliCmd::new_help_command(),
            "q" | "quit" => CliCmd::new_quit_command(),
            _ => CliCmd::new_unknown_command(),
        }
    }

    fn new_push_command(number: f64) -> CliCmd {
        CliCmd {
            oper: CliOperation::Push(number),
        }
    }

    fn new_add_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Add,
        }
    }

    fn new_multiply_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Multiply,
        }
    }

    fn new_divide_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Divide,
        }
    }

    fn new_subtract_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Subtract,
        }
    }

    fn new_clear_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Clear,
        }
    }

    fn new_list_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::List,
        }
    }

    fn new_help_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Help,
        }
    }

    fn new_quit_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Quit,
        }
    }

    fn new_unknown_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Unknown,
        }
    }

    fn new_empty_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::Empty,
        }
    }
}

pub struct Cli {
    keep_running: bool,
    cursor_character: char,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            keep_running: true,
            cursor_character: '>',
        }
    }

    fn display_command_output(&self, cmd: CliOperation) {
        match cmd {
            CliOperation::Unknown => {
                println!("Unknown command - 'help' for a list of commands");
            }
            CliOperation::Quit => {}
            CliOperation::Help => {
                println!("Commands:");
                println!("  <number>\tPush a number to the stack");
                println!("  + / a\t\tAdd the top two numbers from the stack");
                println!("  - / s\t\tSubtract the top two number from the stack");
                println!("  * / x\t\tMultiply the top two numbers from the stack");
                println!("  / / d\t\tDivide the top two numbers from the stack");
                println!("  print / p\tDisplay the stack");
                println!("  help / h:\tDisplay this message");
                println!("  quit / q:\tQuit the program");
            }
            _ => {}
        }
    }

    pub fn keep_running(&self) -> bool {
        self.keep_running
    }

    pub fn read_new_command<R>(&mut self, reader: R) -> CliCmd
    where
        R: io::BufRead,
    {
        self.display();
        let cmd = CliCmd::from_raw_command(self.get_raw_cmd_from_user(reader));
        if cmd.oper == CliOperation::Quit {
            self.keep_running = false;
        }

        match cmd.oper {
            CliOperation::Unknown => self.display_command_output(CliOperation::Unknown),
            CliOperation::Quit => self.display_command_output(CliOperation::Quit),
            CliOperation::Help => self.display_command_output(CliOperation::Help),
            _ => {}
        }

        cmd
    }

    fn display(&self) {
        print!("{} ", self.cursor_character);
        std::io::stdout().flush().unwrap();
    }

    fn get_raw_cmd_from_user<R>(&self, mut reader: R) -> String
    where
        R: io::BufRead,
    {
        let mut raw_cmd = String::new();
        reader
            .read_line(&mut raw_cmd)
            .expect("Error reading command");
        raw_cmd
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
