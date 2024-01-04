
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a/b)
    }
}

fn percentage(percent: f64, number: f64) -> f64 {
    (percent/100.0) * number
}

use std::f64::consts::PI;

fn cos(angle_in_degrees: f64) -> f64 {
    (angle_in_degrees * PI / 180.0).cos()
}

fn sin(angle_in_degrees: f64) -> f64 {
    (angle_in_degrees * PI / 180.0).sin()
}

fn tan(angle_in_degrees: f64) -> f64 {
    (angle_in_degrees * PI / 180.0).tan()
}

fn exponentiate(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}



use std::io;


fn main() {
    println!("Hey, this simple calculator. Welcome! (Still working on it)");

    loop {
        let mut input = String::new();

        println!("Please enter the operation (add, subtract, multiply, divide, percentage, cos, sin, tan, power) or 'exit' to quit:");
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        if input.trim() == "exit" {
            break;
        }

        match input.trim() {
            "add" | "subtract" | "multiply" | "divide" | "percentage" | "power" => {
                let num1 = read_number("Please type a first number:");
                let num2 = read_number("Please type a second number:");
                perform_double_input_operation(input.trim(), num1, num2);
            },

            "cos" | "sin" | "tan" => {
                let num = read_number("Enter the angle in degrees:");
                perform_single_input_operation(input.trim(), num)
            }
            _ => println!("Invalid operation!"),
        }
    }
}


fn read_number(prompt: &str) -> f64 {
    let mut num = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut num).expect("Failed to read input.");
    num.trim().parse().expect("Please type a number.")
}

fn perform_double_input_operation(operation: &str, num1: f64, num2: f64) {
    match operation {
        "add" => println!("Result: {:.2}", add(num1, num2)),
            "subtract" => println!("Result: {:.2}", subtract(num1, num2)),
            "multiply" => println!("Result: {:.2}", multiply(num1, num2)),
            "divide" => match divide(num1, num2) {
                Some(result) => println!("Result: {:.2}", result),
                None => println!("Cannot divide by zero."),
            },
            "percentage" => println!("Result: {:.2}", percentage(num1, num2)),
            "power" => println!("Result: {:.2}", exponentiate(num1, num2)),
            _ => {}
    }
}

fn perform_single_input_operation(operation: &str, num: f64) {
    match operation {
        "cos" => println!("Result: {:.2}", cos(num)),
        "sin" => println!("Result: {:.2}", sin(num)), 
        "tan" => println!("Result: {:.2}", tan(num)),
        _ => {}
    }
}