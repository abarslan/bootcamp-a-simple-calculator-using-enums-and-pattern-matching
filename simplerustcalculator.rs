use std::io;

enum operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate(x: operation) -> f64 {
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
let first_number: f64 = a.trim().parse().unwrap();
println!("Enter the operation: ");
let mut op = String::new();
io::stdin().read_line(&mut op);
let operation: char = op.trim().chars().next().unwrap();
println!("Enter the second number: ");
let mut b = String::new();
io::stdin().read_line(&mut b);
let second_number: f64 = b.trim().parse().unwrap();

let operation_enum = match operation {
    '+' => operation::Add(first_number, second_number),
    '-' => operation::Subtract(first_number, second_number),
    '*' => operation::Multiply(first_number, second_number),
    '/' => operation::Divide(first_number, second_number),
    _ => panic!("Invalid operation"),};

    let result = calculate(operation_enum);
    println!("The result is: {}", result);
}