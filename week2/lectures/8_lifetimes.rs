// TODO 'a should check again
struct Somethings<'a> {
    a: &'a Vec<u8>,

}

fn main(){
    let my_var : Vec<u8> = vec![1,2];
    let something = Somethings {a:&my_var};
    
    println!("{:?}",something.a); // Non-Lexical Lifetimes means last use of something it gets dropped
    println!("{:?}",something.a); // Non-Lexical Lifetimes means last use of something it gets dropped
    drop(my_var);
}
