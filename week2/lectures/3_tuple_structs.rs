
/** TUPLE STRUCTS
*  A struct * Tuple that has unnamed fields
*
*/



struct User {
    name:String,
    age:u8,
    additonal_info:Option<Vec<String>>,
}


// Apply tuple stcut bypass below limition
// tood check. how to use generatic type.
// impl Vec<T> {
//     fn my_func(){
//         println!("my func");
//     }
// }

struct Wrapper<User>(Vec<User>);


impl Wrapper<User> {
    fn my_func(&self){
        println!("{:?}", (self.0)[0].name)
    }
}

fn main(){
    let mut alice: User = User {
        name: String::from("Alice"),
        age:30,
        additonal_info:None,
    };

    let vec:Vec<User> = vec![alice];
    let wrapper:Wrapper<User> = Wrapper(vec);
    wrapper.my_func();
   




}
