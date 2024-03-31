// Define a struct named 'Person'
struct Person {
    name: String,
    age: u32,
}

// Implementation block for the 'Person' struct
impl Person {
    // Constructor method to create a new 'Person' instance
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    // Method to greet the person
    fn greet(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

pub fn show_structs_result() {
    // Create an instance of 'Person' using the constructor method
    let person1 = Person::new("Alice", 30);

    // Access struct fields and call struct methods
    println!("Name: {}, Age: {}", person1.name, person1.age);
    person1.greet();

    // Create another instance of 'Person'
    let person2 = Person {
        name: "Bob".to_string(),
        age: 25,
    };

    // Access struct fields and call struct methods
    println!("Name: {}, Age: {}", person2.name, person2.age);
    person2.greet();
}
