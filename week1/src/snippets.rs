
fn main() {
    
//    snippet_1();
//    snippet_2();
//    snippet_3();
//    snippet_4();
      snippet_5();
}

// fn snippet_1() {
//     // Tuples
//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is: {y}");

//     let tup: (i32, f64, u128) = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is: {y}");

//     // Arrays (Fixed length)
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     for elem in a {
//         println!("a has element: {}", elem);
//     }
// }

// fn snippet_2() {
//     let mut v = vec![1, 2, 3];
//     for elem in v { // 1. for loop scop had got v ownership, following v can't be used.
//         println!("v element: {elem}");
//     }

//     // 2. v default is immutable. can't change
//     v.push(1234); // what is wrong?
// }


fn snippet_3() {
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary. 
    // This is also why string literals are immutable; &str is an immutable reference.
    // let s = "Hello, world";

    let s = String::from("hello world");

    // see https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices
    let hello = &s[0..=2];
    let world = &s[6..11];
    println!("{}",hello);
    println!("{}",world);
}



fn snippet_4() {
    let variable: String = String::from("Welcome to RustSkills");

    example(&variable);

    println!("In main, variable is: {}", variable); // 
}

fn example(var: &String) {
    println!("variable is: {}", var);
}


use std::{collections::HashMap};

fn snippet_5() {
    let mut test: HashMap<i32, HashMap<_,_>> = HashMap::new();

    let mut test2: HashMap<i32, f32> = HashMap::new();

    test.insert(1, test2);

    let test2 = test.get_mut(&1).unwrap();
    test2.insert(123, 123.4);
    test.get_mut(&1).unwrap().insert(1, 99.4);

    println!("test: {:?}", test.get(&1).unwrap());
    println!("test: {:?}", test.get(&2));
}


/**
 * https://doc.rust-lang.org/book/ch08-03-hash-maps.html
 * 
 * tood more practice in the future. 20250712
 * 
 */

 /**
  * Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
  * and mode (the value that occurs most often; a hash map will be helpful here) of the list.
  * 
  */

 fn _collection_practice1() -> (u32, u32, u32) {

    let vec:Vec<u32> = vec![1,1,9,23,23,23,45];
    let hashmap:HashMap<u32,u32> = HashMap::new();
    for i in 0..vec.len() {

        hashmap.insert(i as u32, vec[i]);
    }

    if vec.len() %2 == 0 {

    } else {

    }
    let median = vec[]
    




 }
