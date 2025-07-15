

/////////////////////////////////////////////////////////snippet_4////////////////////////////////////////////////////////////

/**
 *  Code analysis
 * 
 *  What is the main difference between the two languages about non-initialized data?
 * 
 *    1. solidity will directly get the expected data(default transferring to the expected data type). if no value, apply the type default vaule.
 * 
 *    2. Rust:  
 *             1.Instead of returning the vaule. rust return the Option<&V>, which should be convert to the exepected value by calling unwrap. 
 *                Also support iterator(for loop) usage. solidity doesn't support this feature.
 *             2. Supply more flexible api, such as entry judging whether or not one value is exist, which makes some complex code more clean and simple
 *             3. Shoul keep in mind the immutable and reference concpet. 
 * 
 *
 *  
 * 
 *  For storage? (TO DO confirm)
 *  Solidity:
 *      Calculate the actual value(key=>value) storage index based on the mapping variable storage index and key, then store the value in the storage index.
 *      For example: below solidity code `values` in storage index is 0. user input("hello",256) then keccak256(0,"hello"), which is the storge index for 256.
 * 
 * 
 *  Rust (TODO)
 *         let mut values: HashMap<String, u64> = HashMap::new();
 *          stack: store the pointer (keys pointer and keys metadata/ values pointer and values metadata) 
 *          heap: acutal data for all keys and all values.
 * 
 * 
 */


use std::collections::HashMap;

fn main() {
    let mut values: HashMap<String, u64> = HashMap::new();

    values.insert(String::from("test"), 12345);
   //  values.insert(String::from("test1"), 123456);

    
     //  input "test"  can also works 
    println!("\"test\" associated value is {}", values.get(&String::from("test1")).unwrap());

   //  for (key,value) in values{
   //    println!("{key}:{value}")
   //  }
}





// // SPDX-License-Identifier: GPL-3.0

// pragma solidity ^0.8.0;

// contract TestMapping {

//     mapping(string => uint256) values;

//     function add(string calldata input, uint256 value) external {
//         values[input] = value;
//     }

//     function read(string calldata input) external view returns (uint256) {
//         return values[input];
//     }
// }


/////////////////////////////////////////////////////////snippet_3////////////////////////////////////////////////////////////

// x input 3.
// this x is corrospending to 'let mut x: i32 = 2;' which can be changed.
// two change places. one 'x = x ^ 2' ==>0  another 'x = 3' in inner scope.
// finally println scope is the same as  'let mut x: i32 = 2'.
fn snippet_3() {
    let  x: i32 = 1;
    {   
        let mut x: i32 = 2;
        x = x ^ 2;

        {
            x = 3;
            let x = 12;
        }
        println!("x is: {}", x);
    }
}



///////////////////////////////////////////////////////////snippet_2////////////////////////////////////////////////////////


fn snippet_2() {

    let mut num1 = 1234;
    let num2 = 1122;
    println!("My result is {}!", my_operation(num1, num2));
    println!("{num1},{num2}!");
}
fn my_operation(mut a: u64, b: u64) -> u64 {
    a += b;
    a
}
// num1 default type is immutable, It can't be change when it was transferred to my_operation.
// if want to change, should make num1 as mut. 


// fn _snippet_2() {

//     let num1 = 1234;
//     let num2 = 1122;
//     println!("My result is {}!", my_operation(num1, num2));
// }
// fn _my_operation(a: u64, b: u64) -> u64 {
//     a += b;
//     a
// }

//////////////////////////////////////////////////////////// snippet_1 ///////////////////////////////////////////////////////////

fn snippet_1() {
    let mut  a = vec![1,2,3,4];
    // all the variable default type is immutable. can't change. 
    // should statement a as mut.
    a.push(27);
}