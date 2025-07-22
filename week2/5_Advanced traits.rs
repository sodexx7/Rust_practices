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
 * Advanced traits
 * 
 * Retake the Calculator program from Exercise 1.

    - Add a `print_output` function which needs a single input parameter with the 3 defined traits:  `AdditiveOperations`, `MultiplicativeOperations` , `BinaryOperations`
    - This function should print every operations’ results for the given input.
 * 
 */

// use std::ops::Add;

// struct AnyNumber;


// impl Add<AnyNumber> for AnyNumber {
//     type Output = AnyNumber;

//     fn add(self, other: AnyNumber) -> AnyNumber {
//         self + other
//     }
// }



#[derive(Debug)]
 struct Calculator<T,Y> {
    x: T,
    y: Y,
 }


 trait AdditiveOperations<T,Y> {
    fn addition(&self) -> T;
    fn substraction(&self) -> T;
}

trait MultiplicativeOperations<T,Y> {
    fn multiplication(&self) -> T;
    fn division(&self) -> T;

}

trait BinaryOperations<T,Y> {
    fn and(&self) -> T;
    fn or(&self) -> T;
    fn xor(&self) -> T;
}

impl <T,Y> AdditiveOperations<T,Y> for Calculator<T,Y> {

    fn addition(&self) -> T {
       self.x + self.y
    }
   

    fn substraction(&self) -> T {
        self.x - self.y
    }
}

impl <T,Y> MultiplicativeOperations<T,Y> for Calculator<T,Y> {
    fn multiplication(&self) -> T {
        self.x * self.y
    }

    fn division(&self) -> T {
        self.x / self.y
    }
}

impl <T,Y> BinaryOperations<T,Y> for Calculator<T,Y> {
    fn and(&self) -> T {
        self.x & self.y
    }

    fn or(&self) -> T {
        self.x | self.y
    }

    fn xor(&self) -> T {
        self.x ^ self.y
    }
}


fn print_output(calculator:&(impl AdditiveOperations + MultiplicativeOperations + BinaryOperations)) {
    let mut operations = format!("addition operation {}",calculator.addition());
    operations.push_str(&format!("\n substraction operation {}",calculator.substraction()));
    operations.push_str(&format!("\n multiplication operation {}",calculator.multiplication()));
    operations.push_str(&format!("\n division operation {}",calculator.division()));
    operations.push_str(&format!("\n and operation {}",calculator.and()));
    operations.push_str(&format!("\n or operation {}",calculator.or()));
    operations.push_str(&format!("\n xor operation {}",calculator.xor()));
    println!("{operations}");
}


impl <T,Y> Display for Calculator<T,Y> {
    // How to deal with the math error quesiton. even one operation occur panic, other operations can still operate. ?
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("(x={:?}, y={:?})", self.x, self.y);
        output.push_str(&format!("\n addition {:?}",self.addition()));
        output.push_str(&format!("\n substraction {:?}",self.substraction()));
        output.push_str(&format!("\n multiplication {}",self.multiplication()));
        output.push_str(&format!("\n division {}",self.division()));
        output.push_str(&format!("\n and {}",self.and()));
        output.push_str(&format!("\n or {}",self.or()));
        output.push_str(&format!("\n xor {}",self.xor()));
        write!(f,"{}",output)

    }
   
 }


 fn main(){

    println!("{}",i32::MAX);
    let calculator: Calculator<i32, i32> = Calculator {x:100,y:200};
    println!("calculator: {}", calculator); 
    println!("Apply print_output....................................");
    print_output(&calculator);


 }