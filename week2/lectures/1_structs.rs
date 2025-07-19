/** STRUCTS
 *  Complex aggregate datatype that contains named fields,- stored on the stack
 *  They don't implement Display or DeBUG traits by default
 *  All fileds of the struct must be Sized - expcet the last filed of the struct can be ?Sized(Can be sized or not sized) -DST(Dynamatic Sized Type)
 *      For non-last filed of the struct. if want to use the DST. the solutions?
 *      String as vec dst. we know the size in the stack. (Stack know sized)
 * 
 */


struct User {
    
    name:String,
    age:u8,
    additonal_info:Option<Vec<String>>,
}

fn main(){
    let mut alice = User {
        name: String::from("Alice"),
        age:30,
        additonal_info:None,
    };
    println!("The user is {:?}.",alice);
    println!("The user is {} and is {} years old.",alice.name,alice.age);
    match alice.additonal_info{
        Some(infos) => println!("Additonal info [ {} ]",infos.join(",")),
        None => println!("No additional info for user"),
    }

    alice.additonal_info = Some(vec![String::from("work: Security Research"),String::from("skills: Rust")]);
    println!("The user is {} and is {} years old.",alice.name,alice.age);
    match alice.additonal_info{
        Some(infos) => println!("Additonal info [ {} ]",infos.join(",")),
        None => println!("No additional info for user"),
    }

}
