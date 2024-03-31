pub fn show_loop_result() {
    let mut count = 0;

    // Example 1: Basic loop with conditional break
    println!("Example 1: Basic loop with conditional break");
    loop {
        println!("Count: {}", count);
        count += 1;
        if count >= 5 {
            break;
        }
    }

    // Example 2: Loop with labeled break
    println!("Example 2: Loop with labeled break");
    'outer_loop: loop {
        println!("Outer loop count: {}", count);
        'inner_loop: loop {
            println!("Inner loop count: {}", count);
            count += 1;
            if count >= 10 {
                break 'inner_loop;
            }
        }
        count += 2;
        if count >= 20 {
            break 'outer_loop;
        }
    }

    // Example 3: Loop with continue
    println!("Example 3: Loop with continue");
    count = 0;
    loop {
        count += 1;
        if count % 2 == 0 {
            continue;
        }
        println!("Count: {}", count);
        if count >= 5 {
            break;
        }
    }
}

pub fn show_while_result() {
    let mut count = 0;

    // Example 1: while loop with exit condition
    println!("Example 1: while loop with exit condition");
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // Example 2: Infinite while loop with conditional exit
    println!("Example 2: Infinite while loop with conditional exit");
    let mut number = 0;
    while number < 10 {
        if number % 2 == 0 {
            number += 1;
            continue; // Skip even numbers
        }
        println!("Odd number: {}", number);
        number += 1;
    }

    // Example 3: while loop with condition at the end
    println!("Example 3: while loop with condition at the end");
    let mut value = 10;
    while value != 0 {
        println!("Current value: {}", value);
        value -= 1;
    }

    // Example 4: while loop based on an external condition
    println!("Example 4: while loop based on an external condition");
    let mut is_condition_met = false;
    let mut counter = 0;
    while !is_condition_met {
        println!("Counter: {}", counter);
        if counter == 5 {
            is_condition_met = true;
        }
        counter += 1;
    }
}
