// Definition of a function that divides two numbers and returns a Result
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        // If the divisor is zero, return an error
        Err("Cannot divide by zero!")
    } else {
        // If the divisor is non-zero, return the result of the division
        Ok(a / b)
    }
}

pub fn show_result_result() {
    let numerator = 10;
    let denominator = 0;

    // Calling the divide function
    match divide(numerator, denominator) {
        Ok(result) => println!("The result of the division is: {}", result),
        Err(message) => println!("Error: {}", message),
    }
}

// A function that takes an integer and returns an Option containing its square
fn square(num: i32) -> Option<i32> {
    if num >= 0 {
        Some(num * num) // If num is non-negative, return its square wrapped in Some
    } else {
        None // If num is negative, return None
    }
}

pub fn show_option_result() {
    let x = 5;
    let y = -3;

    // Calling the square function and handling the returned Option
    match square(x) {
        Some(result) => println!("Square of {} is {}", x, result),
        None => println!("Cannot calculate square of negative number"),
    }

    match square(y) {
        Some(result) => println!("Square of {} is {}", y, result),
        None => println!("Cannot calculate square of negative number"),
    }
}
