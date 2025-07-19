
struct User {
    name: String,
    age: u8,
    additional_info: Option<Vec<String>>,
}


fn main() {
    let mut alice = User {
        name: String::from("Alice"),
        age: 30,
        additional_info: None,
    };
    println!("The user is {} and is {} years old.", alice.name, alice.age);
    match alice.additional_info {
        Some(infos) => println!("Additional info [ {} ]", infos.join(",")),
        None => println!("No additional info for user"),        
    }

    alice.additional_info = Some(vec![String::from("work: Security Researcher"), String::from("skills: Rust")]);

    println!("The user is {} and is {} years old.", alice.name, alice.age);
    match alice.additional_info {
        Some(infos) => println!("Additional info [ {} ]", infos.join(", ")),
        None => println!("No additional info for user"),        
    }

}