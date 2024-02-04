fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(msg) => println!("Error: {}", msg),
    }
}