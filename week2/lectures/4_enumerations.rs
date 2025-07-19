
/** ENUMERATIONS
*  Complex datatype for variaton,the different choices in the enum are called Variants.
*  
*
*/

//  can have different types
enum Something {
    Dataless,
    Tuple(u8,i8),
    Struct {
        filed1: String,
        filed2: u8,
    }
}

enum Person {
    User(User),  // Person::User != User
    Admin(Administrator),
}


impl Person {
    fn name(&self) {
        match self {
            Person::User(user) => println!("{}",user.name),
            Person::Admin(administrator) => println!("{:?}",administrator.name),
        }
    }
    
}

// pub enum Option<T> {
//     None,
//     Some(T),
// }


struct User {
    name:String,
    age:u8,
    additonal_info:Option<Vec<String>>,
}

struct Administrator {
    name:String,
    identifier:[u8;32],
}

fn generate_user() -> User {
    User { 
            name: String::from("Alice"), 
            age: 30,
            additonal_info: None
        }
}

fn generate_admin() -> Administrator {
    Administrator {
        name: String::from("Bob"),
        identifier: [0u8;32],
    }
}


fn print_person(person:&Person){
    match person {
        Person::User(user) => {
            println!("The user is {} and is {} years old", user.name,user.age);
            match &user.additonal_info {
                Some(infos) => println!("Additional info [{}]",infos.join(",")),
                None => println!("No additional info for user"),
            }
        },
        Person::Admin(admin) => {
            println!("Administor is {} and his id is: {:?}", admin.name,admin.identifier);
        },
    }
}




fn main(){
    let alice: Person = Person::User(generate_user());
    let mut admin = Person::Admin(generate_admin());

    alice.name();
    admin.name();

    print_person(&alice);
    print_person(&admin);

    //  Deconstuciton
    //  LHS for deconstruction and binding, RHS for the object in memory
    // if let LHS = RHS {

    // }

    if let Person::Admin(a) = &mut admin { // Move semantics
        a.identifier = [1u8;32];
        println!("{:?}",a.identifier);

    } 

    admin.name();
    // print_person(&admin);
    // if let Person::User(a) = alice {
        

    // } 



}
