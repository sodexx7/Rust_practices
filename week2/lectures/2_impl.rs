/** Implementations
 *  impl blocks allow for funcitons and methonds associate with a complex datatype
 *  There are 4 references to the type that the impl block is defined for:
 *      - Self          - Refers to the Type the impl block is defined for  
 *      - self          - Refers to an instance of the Type in memory 
 *      - &self         - Refers to a Reference to an instance of the type in memory
 *      - &mut self     - As above, but with mutabliity permissions
 */


pub struct User {
    name:String,
    age:u8,
    additonal_info:Option<Vec<String>>,
}

pub fn pure_outer(){
    println!("NOT DEPEND ON ANYTHINGS IN MEMORY");
}



impl User {
    fn pure(){ // Actually a pure function in the maths sense - referenceital transparency
        println!("NOT DEPEND ON ANYTHINGS IN MEMORY");
    }

     fn moving(self){ // self:Self  // <--- The User is moved into this method
        println!("my name{}",self.name);
        // End of the scope, drop all owned data,drop self
    }


    fn borrowing(&self){ // self:&Self //
        println!("my age{}",self.age);
    }

    fn mut_borrowing(&mut self){ // self: &mut Self
        self.age += 1;
        println!("my age now{}",self.age);
    }

}

fn main(){
    let mut alice: User = User {
        name: String::from("Alice"),
        age:30,
        additonal_info:None,
    };
    println!("The user is {} and is {} years old.",alice.name,alice.age);

    User::pure();
    pure_outer();
    alice.borrowing();
    alice.mut_borrowing();
    alice.moving(); // User : moving(self:alice)
    // alice.moving(); <-- will error as alice was dropped by moving




}
