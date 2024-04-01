pub fn show_closure_result() {
    // Basic example of a closure
    let add_one = |x| x + 1;
    println!("Add one: {}", add_one(5));

    // Closure capturing environment variables
    let mut y = 10;
    let mut add_y = |x| {
        y += 1;
        x + y
    };
    println!("Add y: {}", add_y(5));

    // Capturing environment variables with type specification
    let mut z = 5;
    let mut add_z = |x| {
        z += 1;
        x + z
    };
    println!("Add z: {}", add_z(5));

    // Closure as an argument to a function
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().map(|x| x * 2).sum();
    println!("Sum: {}", sum);

    // Closure as return of a function
    fn make_adder(y: i32) -> impl Fn(i32) -> i32 {
        move |x| x + y
    }
    let add_five = make_adder(5);
    println!("Add five: {}", add_five(10));
}

pub fn show_closure_ownership_result() {
    let x = vec![1, 2, 3, 4, 5];
    let y = 10;

    // Closure that takes ownership of 'x' and 'y', and prints their contents and the sum of all elements
    let print_and_sum = move || {
        let sum: i32 = x.iter().sum();
        println!("Vector contents: {:?}", x);
        println!("Sum of vector elements: {}", sum);
        println!("Value of y: {}", y);
    };

    // Calling the closure
    print_and_sum();

    // This would cause a compilation error because 'x' and 'y' are no longer accessible here
    // println!("Vector contents: {:?}", x);
    // println!("Value of y: {}", y);
}
