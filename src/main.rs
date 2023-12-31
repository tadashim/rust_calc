enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add(a, b) => a + b,
            Operation::Subtract(a, b) => a - b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => a / b,
        }
    }
}

fn main() {
    println!("Enter the first number: ");
    let mut first_number = String::new();
    std::io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

    println!("Enter the operation: ");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation: char = operation.trim().parse().expect("Please type a number!");
    
    println!("Enter the second number: ");
    let mut second_number = String::new();
    std::io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

    // Create an instance of the Operation enum based on the user input.
    // Call the calculate method on the Operation instance.
    // Print the result to the terminal.
    let op = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operation"),
    };
    let result = op.calculate();
    println!("The result is {}", result);
}
