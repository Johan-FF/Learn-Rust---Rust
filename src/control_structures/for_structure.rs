pub fn show_for_result() {
    let array = [1, 2, 3, 4, 5];

    // Example 1: Iterating over an array
    println!("Iterating over an array:");
    for element in &array {
        println!("{}", element);
    }

    // Example 2: Iterating over a range
    println!("Iterating over a range:");
    for i in 0..5 {
        println!("{}", i);
    }

    // Example 3: Iterating over a range with a step
    println!("Iterating over a range with a step:");
    for i in (0..10).step_by(2) {
        println!("{}", i);
    }

    // Example 4: Iterating over characters in a string
    let s = String::from("hello");
    println!("Iterating over characters in a string:");
    for c in s.chars() {
        println!("{}", c);
    }
}
