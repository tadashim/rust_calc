enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> Option<f64> {
        match self {
            Operation::Add(a, b) => Some(a + b),
            Operation::Subtract(a, b) => Some(a - b),
            Operation::Multiply(a, b) => Some(a * b),
            Operation::Divide(a, b) => if *b != 0.0 { Some(a / b) } else { None },
        }
    }
}

fn main() {
    let first_number = read_number("Please enter the first number:");
    let operation = read_operation("Please enter the operation (+, -, * or /):");
    let second_number = read_number("Please enter the second number:");

    let op = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        },
    };

    match op.calculate() {
        Some(result) => println!("The result is {}", result),
        None => println!("Cannot divide by zero"),
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please type a number!"),
        }
    }
}

fn read_operation(prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        let mut operation = String::new();
        std::io::stdin().read_line(&mut operation).expect("Failed to read line");
        match operation.trim().parse() {
            Ok(op) => return op,
            Err(_) => println!("Please type a valid operation (+, -, *, /)"),
        }
    }
}
