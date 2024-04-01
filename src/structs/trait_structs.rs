// Definition of a trait
trait Shape {
    fn area(&self) -> f64;
}

// Implementation of the trait for a circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Implementation of the trait for a rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Function that calculates the area of any shape implementing the Shape trait
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

pub fn show_trait_result() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    // Call to the print_area function with different types of shapes
    print_area(&circle);
    print_area(&rectangle);
}
