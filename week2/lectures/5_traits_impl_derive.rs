
use std::fmt::{Debug, Display, Error};
/** TRAITS
*  They allow us to have opt in  shared behaviour to a complex datatype
*  
*
*/

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(),Error> {
        // todo!();
        write!(f,"I am {} and my age is {}",self.name,self.age)
        // Ok(())
    }
}

// impl Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("user").field("name", &self.name).field("age", &self.age).field("additional info", &self.additonal_info).finish()
//     }

// }
#[derive(Debug)] // When complie, supply above funciton to the body
struct User {
    name:String,
    age:u8,
    additonal_info:Option<Vec<String>>,
}

impl User {
    fn display_data(&self) {
        println!("The user is {} and is {} years old",self.name,self.age);
    }
}

fn main(){
    let alice = User {
        name:String::from("Alice"),
        age:30,
        additonal_info:Some(vec![String::from("work:Security Research"),String::from("skills:Rust")]),
    };

    // User custom function
    alice.display_data();

    // We Want:
    // println!("DISPLAY:{}",alice.display_data());
    println!("DISPLAY:{}",alice);
    println!("Debug:{:?}",alice);



}
