fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let a = 10.0;
    let b = 2.0;
    let division_result = divide(a, b);

    match division_result {
        Ok(value) => println!("{} divide by {} is: {}", a, b, value),
        Err(error_message) => println!("Error: {}", error_message),
    }
}
