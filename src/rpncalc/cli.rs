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
    SquareRoot,
    AddAll,
    MultAll,
    Clear,
    List,
    Quit,
    Unknown,
    Help,
    ClearScreen,
    Empty,
}

pub struct CliCmd {
    pub oper: CliOperation,
}

impl CliCmd {
    fn parse_individual_raw_command(s: &str) -> CliCmd {
        if s.parse::<f64>().is_ok() {
            return CliCmd::new_push_command(f64::from_str(&s).unwrap());
        }
        match s.to_lowercase().as_str() {
            "+" | "a" | "add" => CliCmd::new_add_command(),
            "-" | "s" | "sub" => CliCmd::new_subtract_command(),
            "*" | "x" | "mul" => CliCmd::new_multiply_command(),
            "/" | "d" | "div" => CliCmd::new_divide_command(),
            "sqrt" => CliCmd::new_square_root_command(),
            "++" | "aa" => CliCmd::new_add_all_command(),
            "**" | "xx" => CliCmd::new_mult_all_command(),
            "c" | "clear" => CliCmd::new_clear_command(),
            "p" | "print" => CliCmd::new_list_command(),
            "h" | "help" => CliCmd::new_help_command(),
            "q" | "quit" => CliCmd::new_quit_command(),
            "cls" => CliCmd::new_clear_screen_command(),
            _ => CliCmd::new_unknown_command(),
        }
    }

    fn from_raw_command(s: String) -> Vec<CliCmd> {
        if s.is_empty() {
            // Its an EOF, Ctrl+D string
            return vec![CliCmd::new_quit_command()];
        }

        let tokenized_command = s.split_whitespace().collect::<Vec<_>>();

        if tokenized_command.is_empty() {
            return vec![CliCmd::new_empty_command()];
        }

        let mut commands: Vec<CliCmd> = vec![];
        for raw_command in tokenized_command.iter() {
            commands.push(CliCmd::parse_individual_raw_command(raw_command));
        }
        return commands;
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

    fn new_square_root_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::SquareRoot,
        }
    }

    fn new_add_all_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::AddAll,
        }
    }

    fn new_mult_all_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::MultAll,
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

    fn new_clear_screen_command() -> CliCmd {
        CliCmd {
            oper: CliOperation::ClearScreen,
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

    fn help_message(&self) {
        println!("Commands:");
        println!("  <number>\t\tPush a number to the stack");
        println!("  + a add\t\tAdd the top two numbers from the stack");
        println!("  - s sub\t\tSubtract the top two number from the stack");
        println!("  * x mul\t\tMultiply the top two numbers from the stack");
        println!("  / d div\t\tDivide the top two numbers from the stack");
        println!("  sqrt\t\t\tCalculate the square root of the top of the stack");
        println!("  ++ aa\t\t\tSum all the stack");
        println!("  ** xx\t\t\tMultiply all the stack");
        println!("  c clear\t\tClear the stack");
        println!("  p print\t\tDisplay the stack");
        println!("  h help:\t\tDisplay this message");
        println!("  cls:\t\t\tClear the cli screen");
        println!("  q quit:\t\tQuit the program");
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn display_command_output(&self, cmd: &CliCmd) {
        match cmd.oper {
            CliOperation::Unknown => {
                println!("Unknown command - 'help' for a list of commands");
            }
            CliOperation::Quit => {
                println!("Exiting");
            }
            CliOperation::ClearScreen => {
                self.clear_screen();
            }
            CliOperation::Help => {
                self.help_message();
            }
            _ => {}
        }
    }

    pub fn keep_running(&self) -> bool {
        self.keep_running
    }

    pub fn read_new_command<R>(&mut self, reader: R) -> Vec<CliCmd>
    where
        R: io::BufRead,
    {
        self.display();
        let cmds = CliCmd::from_raw_command(self.get_raw_cmd_from_user(reader));
        if cmds[0].oper == CliOperation::Quit {
            self.keep_running = false;
        }

        self.display_command_output(&cmds[0]);

        cmds
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
