mod calculator;
mod cli;

// Public API

pub struct RpnCalc {
    stack: Vec<f64>,
}

impl RpnCalc {
    pub fn new() -> RpnCalc {
        RpnCalc { stack: vec![] }
    }

    pub fn cli() -> cli::Cli {
        cli::Cli::new()
    }

    pub fn process(&mut self, cmd: cli::CliCmd) {
        calculator::process(self, cmd)
    }
}

impl Default for RpnCalc {
    fn default() -> Self {
        Self::new()
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn process_command(calc: &mut RpnCalc, cmd: &str) {
        let mut cli = RpnCalc::cli();
        let command = std::io::Cursor::new(cmd);
        calc.process(cli.read_new_command(command));
    }

    #[test]
    fn start_new_calculator() {
        let calc = RpnCalc::new();
        assert_eq!(true, calc.stack.is_empty());
        let cli = RpnCalc::cli();
        assert_eq!(cli.keep_running(), true);
    }

    #[test]
    fn cli_exit_calculator() {
        let mut cli = RpnCalc::cli();
        assert_eq!(cli.keep_running(), true);
        let command = std::io::Cursor::new(b"quit");
        cli.read_new_command(command);
        assert_eq!(cli.keep_running(), false);

        let mut cli = RpnCalc::cli();
        assert_eq!(cli.keep_running(), true);
        let command = std::io::Cursor::new(b"q");
        cli.read_new_command(command);
        assert_eq!(cli.keep_running(), false);
    }

    #[test]
    fn cli_exit_calculator_using_eof() {
        let mut cli = RpnCalc::cli();
        assert_eq!(cli.keep_running(), true);
        let command = std::io::Cursor::new(b"quit");
        cli.read_new_command(command);
        assert_eq!(cli.keep_running(), false);

        let mut cli = RpnCalc::cli();
        assert_eq!(cli.keep_running(), true);
        let command = std::io::Cursor::new(b"");
        cli.read_new_command(command);
        assert_eq!(cli.keep_running(), false);
    }

    #[test]
    fn cli_push_valid_numbers() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "0");
        process_command(&mut calc, "-1");
        process_command(&mut calc, "-1.6");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1.9");
        process_command(&mut calc, "+4");
        process_command(&mut calc, "+10.8");
        process_command(&mut calc, "-1.5e1");
        process_command(&mut calc, "2.5e4");
        process_command(&mut calc, "+1e6");
        process_command(&mut calc, ".5");
        assert_eq!(
            calc.stack,
            [0.0, -1.0, -1.6, 1.0, 1.9, 4.0, 10.8, -15.0, 25000.0, 1000000.0, 0.5]
        );
    }

    #[test]
    fn cli_do_not_push_invalid_numbers() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "pi");
        process_command(&mut calc, "e");
        process_command(&mut calc, "log10");
        process_command(&mut calc, "zero");
        process_command(&mut calc, "x");
        process_command(&mut calc, "xx");
        process_command(&mut calc, "1,1");
        process_command(&mut calc, "1e");
        process_command(&mut calc, "1-");
        process_command(&mut calc, "10-1");
        process_command(&mut calc, "foo");
        process_command(&mut calc, "++1");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_clear_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        assert_eq!(calc.stack, [1.0, 1.0, 1.0, 1.0]);
        process_command(&mut calc, "c");
        assert_eq!(calc.stack, []);

        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        assert_eq!(calc.stack, [1.0, 1.0, 1.0, 1.0]);
        process_command(&mut calc, "clear");
        assert_eq!(calc.stack, []);

        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        assert_eq!(calc.stack, [1.0, 1.0, 1.0, 1.0]);
        process_command(&mut calc, "C");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_add_operation_1() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "+");
        assert_eq!(calc.stack, [2.0]);
    }

    #[test]
    fn cli_add_operation_2() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "-3.5");
        process_command(&mut calc, "+3.5");
        process_command(&mut calc, "+");
        assert_eq!(calc.stack, [0.0]);
    }

    #[test]
    fn cli_add_empty_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "+");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_add_single_element_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "10");
        process_command(&mut calc, "+");
        assert_eq!(calc.stack, [10.0]);
    }

    #[test]
    fn cli_add_top_of_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "2");
        process_command(&mut calc, "3");
        process_command(&mut calc, "4");
        process_command(&mut calc, "5");
        process_command(&mut calc, "+");
        assert_eq!(calc.stack, [1.0, 2.0, 3.0, 9.0]);
        process_command(&mut calc, "a");
        assert_eq!(calc.stack, [1.0, 2.0, 12.0]);
        process_command(&mut calc, "A");
        assert_eq!(calc.stack, [1.0, 14.0]);
        process_command(&mut calc, "add");
        assert_eq!(calc.stack, [15.0]);
    }

    #[test]
    fn cli_sub_operation_1() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "1");
        process_command(&mut calc, "-");
        assert_eq!(calc.stack, [0.0]);
    }

    #[test]
    fn cli_sub_operation_2() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "10");
        process_command(&mut calc, "3");
        process_command(&mut calc, "-");
        assert_eq!(calc.stack, [7.0]);
    }

    #[test]
    fn cli_sub_empty_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "-");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_sub_single_element_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "10");
        process_command(&mut calc, "-");
        assert_eq!(calc.stack, [10.0]);
    }

    #[test]
    fn cli_sub_top_of_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "200");
        process_command(&mut calc, "100");
        process_command(&mut calc, "50");
        process_command(&mut calc, "25");
        process_command(&mut calc, "5");
        process_command(&mut calc, "-");
        assert_eq!(calc.stack, [200.0, 100.0, 50.0, 20.0]);
        process_command(&mut calc, "s");
        assert_eq!(calc.stack, [200.0, 100.0, 30.0]);
        process_command(&mut calc, "S");
        assert_eq!(calc.stack, [200.0, 70.0]);
        process_command(&mut calc, "sub");
        assert_eq!(calc.stack, [130.0]);
    }

    #[test]
    fn cli_mult_operation_1() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "-2");
        process_command(&mut calc, "3");
        process_command(&mut calc, "*");
        assert_eq!(calc.stack, [-6.0]);
    }

    #[test]
    fn cli_mult_operation_2() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1e2");
        process_command(&mut calc, "3");
        process_command(&mut calc, "x");
        assert_eq!(calc.stack, [300.0]);
    }

    #[test]
    fn cli_mult_empty_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "*");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_mult_single_element_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "10");
        process_command(&mut calc, "*");
        assert_eq!(calc.stack, [10.0]);
    }

    #[test]
    fn cli_mult_top_of_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "2");
        process_command(&mut calc, "3");
        process_command(&mut calc, "4");
        process_command(&mut calc, "5");
        process_command(&mut calc, "*");
        assert_eq!(calc.stack, [1.0, 2.0, 3.0, 20.0]);
        process_command(&mut calc, "x");
        assert_eq!(calc.stack, [1.0, 2.0, 60.0]);
        process_command(&mut calc, "X");
        assert_eq!(calc.stack, [1.0, 120.0]);
        process_command(&mut calc, "mul");
        assert_eq!(calc.stack, [120.0]);
    }

    #[test]
    fn cli_div_operation_1() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "-1e6");
        process_command(&mut calc, "10");
        process_command(&mut calc, "/");
        assert_eq!(calc.stack, [-1e5]);
    }

    #[test]
    fn cli_div_operation_2() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "200");
        process_command(&mut calc, ".5");
        process_command(&mut calc, "d");
        assert_eq!(calc.stack, [400.0]);
    }

    #[test]
    fn cli_div_empty_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "/");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_div_single_element_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "10");
        process_command(&mut calc, "/");
        assert_eq!(calc.stack, [10.0]);
    }

    #[test]
    fn cli_div_top_of_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "2048");
        process_command(&mut calc, "1024");
        process_command(&mut calc, "512");
        process_command(&mut calc, "256");
        process_command(&mut calc, "128");
        process_command(&mut calc, "/");
        assert_eq!(calc.stack, [2048.0, 1024.0, 512.0, 2.0]);
        process_command(&mut calc, "d");
        assert_eq!(calc.stack, [2048.0, 1024.0, 256.0]);
        process_command(&mut calc, "D");
        assert_eq!(calc.stack, [2048.0, 4.0]);
        process_command(&mut calc, "div");
        assert_eq!(calc.stack, [512.0]);
    }

    #[test]
    fn cli_div_zero_division() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "10");
        process_command(&mut calc, "0");
        process_command(&mut calc, "/");
        assert_eq!(calc.stack, [10.0, 0.0]);
    }

    #[test]
    fn cli_add_all_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "2");
        process_command(&mut calc, "3");
        process_command(&mut calc, "4");
        process_command(&mut calc, "5");
        process_command(&mut calc, "++");
        assert_eq!(calc.stack, [15.0]);
    }

    #[test]
    fn cli_add_all_stack_2() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "2");
        process_command(&mut calc, "aa");
        assert_eq!(calc.stack, [3.0]);
    }

    #[test]
    fn cli_add_all_stack_3() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "2");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "0");
        process_command(&mut calc, "aa");
        assert_eq!(calc.stack, [3.0]);
    }

    #[test]
    fn cli_add_all_empty_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "++");
        assert_eq!(calc.stack, []);
    }

    #[test]
    fn cli_add_all_single_element_stack() {
        let mut calc = RpnCalc::new();
        process_command(&mut calc, "1");
        process_command(&mut calc, "++");
        assert_eq!(calc.stack, [1.0]);
    }
}
