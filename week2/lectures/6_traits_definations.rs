
use std::fmt::Display;


/** TRAITS DEFINATION
*  alice.action() <-- function or method because of ()
*  alice.action <-- field of struct as no parenthesis
*. Default bodies can be provided, and optionally overwritten for traits.
*/


struct User {
    name:String,
    age:u8,
    action:String,
    confidential_info:Option<Vec<String>>,
}

trait Action {
    fn action(&self) -> String {
        String::from("DEFAULT!!")
    }
}

impl Action for User {
    fn action(&self) -> String {
        let mut action = format!("What does{} do for a living",self.name);
        action.push_str(&format!("{} {},",self.name,self.action));
        action
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.confidential_info {
            Some(_) => write!(f,"{} is {} years old,{} and has confidential data.",self.name,self.age,self.action),
            None => write!(f,"{} is {} years old and {}.",self.name,self.age,self.action),
        }
    }
}



fn main(){
    let mut alice = generate_user();
    let alice_action: String = alice.action();
    println!("{}",alice_action);
    // println!("{}",alice.action);
    // println!("{}",alice);

    // alice.confidential_info = Some(vec![String::from("work: SR"),String::from("skills:Rust")]);
    // println!("Output should hcange:{}",alice)



}

fn generate_user() -> User {
    User {
        name: String::from("Alice"),
        age:30,
        action:String::from("breaks code"),
        confidential_info:None,
    }
}
