pub fn show_slices_result() {
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Get a slice of the first 5 elements
    let slice1 = &numbers[0..5];
    println!("Slice 1: {:?}", slice1);

    // Get a slice of elements from 3 to 7
    let slice2 = &numbers[2..7];
    println!("Slice 2: {:?}", slice2);

    // Get a slice from the third element to the end
    let slice3 = &numbers[2..];
    println!("Slice 3: {:?}", slice3);

    // Get a slice from the beginning to the fifth element
    let slice4 = &numbers[..5];
    println!("Slice 4: {:?}", slice4);

    // Modify a slice
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mutable_slice = &mut numbers[2..7];
    for num in mutable_slice.iter_mut() {
        *num *= 2;
    }
    println!("Modified Slice 2: {:?}", mutable_slice);

    // Pass a slice as an argument to a function
    print_slice(slice1);
    print_slice_with_block_labels();
}

// Function to print a slice
fn print_slice(slice: &[i32]) {
    println!("Slice passed to function: {:?}", slice);
}

fn print_slice_with_block_labels() {
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Slices in blocks");

    // Create an outer block label
    'outer: {
        // Create an inner block label
        'inner: {
            // Get a slice of the first three numbers
            let slice = &numbers[0..3];
            println!("Slice: {:?}", slice);

            // Check if the sum of the numbers in the slice is greater than 5
            let mut sum = 0;
            for &num in slice {
                sum += num;
            }

            if sum > 5 {
                // If the sum is greater than 5, break out of the outer block
                break 'outer;
            } else {
                // If the sum is less than or equal to 5, break out of the inner block
                break 'inner;
            }
        }
    }

    println!("End of the code block");
}
