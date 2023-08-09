use std::io;

enum operation { //an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate(x: operation) -> f64 { //function called calculate that takes an Operation enum as an argument and returns an f64 result.
    match x {
        operation::Add(a, b) => a + b,
        operation::Subtract(a, b) => a - b,
        operation::Multiply(a, b) => a * b,
        operation::Divide(a, b) => a / b,
    }
}
fn main(){
println!("Enter the first number: ");
let mut a = String::new();
io::stdin().read_line(&mut a);
let first_number: f64 = a.trim().parse().unwrap();//Parsing the user input into appropriate variables.
println!("Enter the operation: ");
let mut op = String::new();
io::stdin().read_line(&mut op);
let operation: char = op.trim().chars().next().unwrap();//Parsing the user input into appropriate variables.
println!("Enter the second number: ");
let mut b = String::new();
io::stdin().read_line(&mut b);
let second_number: f64 = b.trim().parse().unwrap();//Parsing the user input into appropriate variables.

let operation_enum = match operation { //Operation enum instance with the parsed input values.
    '+' => operation::Add(first_number, second_number),
    '-' => operation::Subtract(first_number, second_number),
    '*' => operation::Multiply(first_number, second_number),
    '/' => operation::Divide(first_number, second_number),
    _ => panic!("Invalid operation"),}; //for covering other chars

    let result = calculate(operation_enum);//called the calculate function for the calculated result
    println!("The result is: {}", result);
}
