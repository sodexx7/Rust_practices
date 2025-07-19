use std::fmt::Display;

struct User {
    name: String,
    age: u8,
    action: String,
    confidential_info: Option<Vec<String>>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.confidential_info {
            Some(_) => write!(f, "{} is {} years old, {} and has confidential data.", self.name, self.age, self.action),
            None => write!(f, "{} is {} years old and {}.",  self.name, self.age, self.action),
        }
    }
}

// Accepts a reference (&) to any type that implement the `Display` trait
fn printer1(var: &impl Display) {
    println!("{}", var);
}

// Trait bounds notation. Particularly used with generic Types.
fn printer2<T: Display> (var: &T) {
    println!("{}", var);
}

fn main() {
    let mut alice = generate_user();

    printer1(&alice);

    alice.confidential_info = Some(vec![String::from("work: SR"), String::from("skills: Rust")]);
    printer2(&alice);
}



// Appendix

fn generate_user() -> User {
    User {
        name: String::from("Alice"),
        age: 30,
        action: String::from("breaks code"),
        confidential_info: None,
    }
}