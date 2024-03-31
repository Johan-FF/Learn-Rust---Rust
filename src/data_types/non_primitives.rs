pub fn show_non_primitive_types() {
    // Strings
    let string_adt: String = String::from("Hello, world!");

    // Arrays
    let array: [i32; 3] = [1, 2, 3];

    // Tuples
    let tuple: (i32, char, f64) = (42, 'a', 3.14);

    // Vectors
    let vector: Vec<i32> = vec![1, 2, 3];

    // Hashmaps
    use std::collections::HashMap;
    let mut hashmap: HashMap<&str, i32> = HashMap::new();
    hashmap.insert("key1", 42);
    hashmap.insert("key2", 84);

    // Print examples
    println!("String Example: {}", string_adt);
    println!("Array Example: {:?}", array);
    println!("Tuple Example: {:?}", tuple);
    println!("Vector Example: {:?}", vector);
    println!("Hashmap Example: {:?}", hashmap);
}
