use std::io;
pub fn main() {
    // Enter nums
    println!("Enter two numbers to perform calculations on:");
    // Get input 1
    let mut input1 = String::new();
    println!("First number:");
    io::stdin().read_line(&mut input1).unwrap();
    // Get input 2
    let mut input2 = String::new();
    println!("Second number:");
    io::stdin().read_line(&mut input2).unwrap();
    // Parse inputs to i32
    let a = input1.trim().parse::<i32>().unwrap();
    let b = input2.trim().parse::<i32>().unwrap();
    // Perform calculations and print results
    println!("{} + {} = {}", a, b, rtb_maths::logic::simple::add(a, b));
    println!("{} - {} = {}", a, b, rtb_maths::logic::simple::subtract(a, b));
    println!("{} * {} = {}", a, b, rtb_maths::logic::simple::multiply(a, b));
    println!("{} / {} = {}", a, b, rtb_maths::logic::simple::divide(a, b));
}