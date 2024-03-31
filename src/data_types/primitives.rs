pub fn show_primitive_types() {
    // Integer data types
    // 8 - 16 - 32 - 64
    let integer: i32 = 2147483647; // -2.147.483.648 <= x <= 2.147.483.647
    let unsigned_integer: u32 = 4294967295; // 0 <= x <= 4.294.967.295
    let long_integer: i64 = 42; // -9.223.372.036.854.775.808 <= x <= 9.223.372.036.854.775.807

    // Floating-point data types
    let float: f32 = 3.4e38; // -3.4 × 10^38 <= x <= 3.4 × 10^38
    let double_float: f64 = -1.7e308; // -1.7 × 10^308 <= x <= 1.7 × 10^308

    // Boolean data type
    let boolean: bool = true;

    // Character data type
    let character: char = 'A';
    let character_unicode: char = '\u{0061}';

    // String
    let string_reference: &str = "Hello, world!";

    // Print primitive data types
    println!("Integer: {}", integer);
    println!("Unsigned Integer: {}", unsigned_integer);
    println!("Long Integer: {}", long_integer);
    println!("Float: {}", float);
    println!("Double Float: {}", double_float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("Character Unicode: {}", character_unicode);
    println!("String: {}", string_reference);
}
