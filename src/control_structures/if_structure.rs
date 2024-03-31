pub fn show_if_result() {
    let number = 6;

    // Example 1: if-else
    if number % 2 == 0 {
        println!("{} is an even number", number);
    } else {
        println!("{} is an odd number", number);
    }

    // Example 2: if with conditional assignment
    let result = if number > 0 { "positive" } else { "negative" };
    println!("The number {} is {}", number, result);

    // Example 3: if-else if-else
    if number < 0 {
        println!("{} is a negative number", number);
    } else if number > 0 {
        println!("{} is a positive number", number);
    } else {
        println!("{} is zero", number);
    }

    // Example 4: if with multiple conditions
    if number % 3 == 0 && (number % 5 == 0 || number % 2 == 0) {
        println!("{} is divisible by 3 and 5 or 2", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 5 == 0 {
        println!("{} is divisible by 5", number);
    } else {
        println!("{} is not divisible by 3 or 5", number);
    }
}
