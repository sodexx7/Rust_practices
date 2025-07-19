use std::fmt::{Display, Debug};

#[derive(Debug)] // See the derive here
struct User {
    name: String,
    age: u8,
    additional_info: Option<Vec<String>>,
}

impl User {
    fn display_data(&self) {
        println!("The user is {} and is {} years old.", self.name, self.age); 
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}

fn main() {
    let alice = User {
        name: String::from("Alice"),
        age: 30,
        additional_info: Some(vec![String::from("work: Security Researcher"), String::from("skills: Rust")]),
    };

    // User custom function
    alice.display_data();

    // Display trait implementation is automatically used
    println!("Display: {}", alice);

    // Debug trait will just go through all element and print it as a structure
    println!("Debug print: {:?}", alice);
}