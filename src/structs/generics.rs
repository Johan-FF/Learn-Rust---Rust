// Definition of a generic structure
struct Pair<T> {
    first: T,
    second: T,
}

// Implementation of methods for the generic structure
impl<T> Pair<T> {
    // Generic constructor
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    // Method that swaps the values
    fn swap(&mut self) {
        std::mem::swap(&mut self.first, &mut self.second);
    }
}

// Generic function that prints the elements of a structure
fn print_pair<T: std::fmt::Debug>(pair: &Pair<T>) {
    println!("Pair: ({:?}, {:?})", pair.first, pair.second);
}

pub fn show_generic_result() {
    // Creating an instance of Pair with integers
    let mut int_pair = Pair::new(1, 2);
    print_pair(&int_pair);

    // Swapping the values in the Pair instance
    int_pair.swap();
    print_pair(&int_pair);

    // Creating an instance of Pair with characters
    let char_pair = Pair::new('a', 'b');
    print_pair(&char_pair);
}
