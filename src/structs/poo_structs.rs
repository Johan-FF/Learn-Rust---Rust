// Define a struct representing a Circle
struct Circle {
    radius: f64,
}

// Implementation block for Circle
impl Circle {
    // Constructor method to create a new Circle
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Method to calculate the area of the Circle
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub fn show_poo_result() {
    // Create a new Circle object
    let circle1 = Circle::new(2.0);

    // Call the area method on the Circle object
    println!("Area of circle: {:.2}", circle1.area());
}
