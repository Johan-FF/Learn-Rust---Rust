pub fn show_match_result() {
    let number = 5;

    // Example 1: Matching a single value
    println!("Example 1: Matching a single value");
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"), // Default case
    }

    // Example 2: Matching multiple values
    println!("Example 2: Matching multiple values");
    let day = "Wednesday";
    match day {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => println!("It's a weekday"),
        "Saturday" | "Sunday" => println!("It's a weekend"),
        _ => println!("Not a valid day"),
    }

    // Example 3: Matching with ranges
    println!("Example 3: Matching with ranges");
    let age = 25;
    match age {
        0..=17 => println!("You're a minor"),
        18..=64 => println!("You're an adult"),
        _ => println!("You're a senior"),
    }

    // Example 4: Matching with destructuring
    println!("Example 4: Matching with destructuring");
    let person = ("John", 30);
    match person {
        ("John", age) => println!("John is {} years old", age),
        (name, age) => println!("{} is {} years old", name, age),
    }

    // Example 5: Matching with guards
    println!("Example 5: Matching with guards");
    let temperature = 20;
    match temperature {
        t if t < 0 => println!("It's freezing"),
        t if t >= 0 && t < 10 => println!("It's cold"),
        t if t >= 10 && t < 20 => println!("It's cool"),
        _ => println!("It's warm"),
    }
}
