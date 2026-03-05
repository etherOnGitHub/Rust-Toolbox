use std::io::{self, Write};
use rtb_maths::logic::simple::{add, subtract, multiply, divide};
use rtb_maths::operators::Operator;

fn get_operand() -> Operator {
    loop {
        println!("Enter an operand (+, -, *, /):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(op) => return op,
            Err(_) => {
                println!("Invalid operand. Please enter +, -, *, or /.");
                continue;
            }
        }
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        // allow programmer to insert strings into the prompt
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    }
}

pub fn main() {
    // Get user input for numbers
    let a = get_number("Enter the first number:");
    let b = get_number("Enter the second number:");
    // Enter operand
    let operator = get_operand();
    // Perform calculations and print results
    match operator {
        Operator::Add => println!("{} + {} = {}", a, b, add(a, b)),
        Operator::Subtract => println!("{} - {} = {}", a, b, subtract(a, b)),
        Operator::Multiply => println!("{} * {} = {}", a, b, multiply(a, b)),
        Operator::Divide => println!("{} / {} = {}", a, b, divide(a, b)),
    }
}
