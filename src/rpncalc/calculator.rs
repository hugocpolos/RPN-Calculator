use super::cli::{CliCmd, CliOperation};
use super::RpnCalc;

pub fn process(c: &mut RpnCalc, cmd: CliCmd) {
    match cmd.oper {
        CliOperation::Push(number) => push(c, number),
        CliOperation::Add => add(c),
        CliOperation::Subtract => subtract(c),
        CliOperation::Multiply => multiply(c),
        CliOperation::Divide => divide(c),
        CliOperation::Clear => clear(c),
        CliOperation::List => list(c),
        _ => {}
    }
}

fn push(c: &mut RpnCalc, number: f64) {
    c.stack.push(number)
}

fn clear(c: &mut RpnCalc) {
    c.stack.clear()
}

fn list(c: &RpnCalc) {
    println!("{:?}", c.stack)
}

fn add(c: &mut RpnCalc) {
    if c.stack.len() < 2 {
        return;
    }

    let result = c.stack.pop().unwrap() + c.stack.pop().unwrap();
    c.stack.push(result);
    print_top(c);
}

fn subtract(c: &mut RpnCalc) {
    if c.stack.len() < 2 {
        return;
    }

    let subtrahend = c.stack.pop().unwrap();
    let minuend = c.stack.pop().unwrap();
    c.stack.push(minuend - subtrahend);
    print_top(c);
}

fn multiply(c: &mut RpnCalc) {
    if c.stack.len() < 2 {
        return;
    }

    let result = c.stack.pop().unwrap() * c.stack.pop().unwrap();
    c.stack.push(result);
    print_top(c);
}

fn divide(c: &mut RpnCalc) {
    if c.stack.len() < 2 {
        return;
    }
    let divisor = c.stack.pop().unwrap();
    let dividend = c.stack.pop().unwrap();
    if divisor == 0.0 {
        println!("Error: Zero division");
        return;
    }
    c.stack.push(dividend / divisor);
    print_top(c);
}

fn print_top(c: &RpnCalc) {
    if c.stack.is_empty() {
        return;
    }
    println!("{0}", c.stack.last().unwrap())
}
