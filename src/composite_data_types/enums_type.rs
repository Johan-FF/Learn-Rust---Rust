// Definition of an enum representing different types of geometric shapes
enum Shape {
    Circle(f64),         // Variant for circles, which includes a radius value
    Rectangle(f64, f64), // Variant for rectangles, which includes width and height values
    Square(f64),         // Variant for squares, which includes a side length value
}

// Function that calculates the area of a geometric shape
fn calculate_area(shape: &Shape) -> f64 {
    return match shape {
        // Pattern matching for each variant of the enum
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side_length) => side_length * side_length,
    };
}

pub fn show_enums_result() {
    // Creating instances of different geometric shapes
    let circle = Shape::Circle(3.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);
    let square = Shape::Square(2.0);

    // Calculating the area of each geometric shape using the enum
    let circle_area = calculate_area(&circle);
    let rectangle_area = calculate_area(&rectangle);
    let square_area = calculate_area(&square);

    // Printing the results
    println!("Area of the circle: {}", circle_area);
    println!("Area of the rectangle: {}", rectangle_area);
    println!("Area of the square: {}", square_area);
}
