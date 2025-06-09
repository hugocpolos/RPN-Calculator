use super::cli::{CliCmd, CliOperation};
use super::RpnCalc;

pub fn process(c: &mut RpnCalc, cmd: &CliCmd) {
    match cmd.oper {
        CliOperation::Push(number) => push(c, number),
        CliOperation::Add => add(c),
        CliOperation::Subtract => subtract(c),
        CliOperation::Multiply => multiply(c),
        CliOperation::Divide => divide(c),
        CliOperation::SquareRoot => square_root(c),
        CliOperation::AddAll => add_all(c),
        CliOperation::MultAll => mult_all(c),
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

    if *c.stack.last().unwrap() == 0.0 {
        println!("Error: Zero division");
        return;
    }

    let divisor = c.stack.pop().unwrap();
    let dividend = c.stack.pop().unwrap();
    c.stack.push(dividend / divisor);
    print_top(c);
}

fn square_root(c: &mut RpnCalc) {
    if c.stack.len() < 1 {
        return;
    }

    if *c.stack.last().unwrap() < 0.0 {
        println!("Error: Negative number square root");
        return;
    }

    let result = c.stack.pop().unwrap().sqrt();
    c.stack.push(result);
    print_top(c);
}

fn add_all(c: &mut RpnCalc) {
    if c.stack.is_empty() {
        return;
    }

    let sum: f64 = c.stack.iter().sum();
    clear(c);
    c.stack.push(sum);
    print_top(c);
}

fn mult_all(c: &mut RpnCalc) {
    if c.stack.is_empty() {
        return;
    }
    let mut mult: f64 = 1.0;
    for i in c.stack.iter() {
        mult *= i;
    }

    clear(c);
    c.stack.push(mult);
    print_top(c);
}

fn print_top(c: &RpnCalc) {
    if c.stack.is_empty() {
        return;
    }
    println!("{0}", c.stack.last().unwrap())
}
