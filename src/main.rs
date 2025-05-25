extern crate rpn_calc;

use rpn_calc::rpncalc;

fn main() {
    let mut my_calc = rpncalc::RpnCalc::new();

    let mut cli = rpncalc::RpnCalc::cli();

    println!("CLI reverse polish notation calculator.");
    println!("'help' for a list of commands");
    while cli.keep_running() {
        my_calc.process(cli.read_new_command(std::io::stdin().lock()));
    }
}
