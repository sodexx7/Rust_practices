use std::{fmt::Display};

/**
 * Build your own "Calculator" Rust program with the following restrictions:
 *   
 * - Create a "Calculator" structure with 2 integer members
 * - The "Calculator" structure should define at least three traits:
 *   - `AdditiveOperations`
 *   - `MultiplicativeOperations`
 *   - `BinaryOperations`
 * - The "Calculator" allow severals operations on scalars:
 *   - Addition
 *   - Substraction
 *   - Multiplication
 *   - Division
 *   - AND
 *   - OR
 *   - XOR
 * - The “Calculator” can be printed through the following line of code `println!("calculator: {}", calculator);`
 *   - When printing the calculator, the result shows the result for each operation.
 * 
 * 
 */

//  for wrapping overflow check can also apply below
use num::CheckedAdd;

 struct Calculator {
    x: i32,
    y: i32,
 }


 trait AdditiveOperations {
    fn addition(&self) -> Option<i32>;
    fn substraction(&self) -> Option<i32>;
}

trait MultiplicativeOperations {
    fn multiplication(&self) -> Option<i32>;
    fn division(&self) -> Option<i32>;

}

trait BinaryOperations {
    fn and(&self) -> i32;
    fn or(&self) -> i32;
    fn xor(&self) -> i32;
}



impl AdditiveOperations for Calculator {
    fn addition(&self) -> Option<i32> {
        self.x.checked_add(self.y)
    }
   

    fn substraction(&self) -> Option<i32> {
        self.x.checked_sub(self.y)
    }
}

impl MultiplicativeOperations for Calculator {
    fn multiplication(&self) -> Option<i32> {
        self.x.checked_mul(self.y)
    }

    fn division(&self) -> Option<i32> {
        self.x.checked_div(self.y)
    }
}

impl BinaryOperations for Calculator {
    fn and(&self) -> i32 {
        self.x & self.y
    }

    fn or(&self) -> i32 {
        self.x | self.y
    }

    fn xor(&self) -> i32 {
        self.x ^ self.y
    }
}


impl Display for Calculator{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("(x={}, y={})", self.x, self.y); 
        output.push_str(&format!("\n addition {:?}",self.addition()));
        output.push_str(&format!("\n substraction {:?}",self.substraction()));
        output.push_str(&format!("\n multiplication {:?}",self.multiplication()));
        output.push_str(&format!("\n division {:?}",self.division()));
        output.push_str(&format!("\n and {}",self.and()));
        output.push_str(&format!("\n or {}",self.or()));
        output.push_str(&format!("\n xor {}",self.xor()));
        write!(f,"{}",output)

    }
   
 }


 fn main(){

    println!("{}",i32::MAX);
    let calculator = Calculator {x:i32::MAX,y:i32::MAX};
    println!("calculator: {}", calculator); 


 }