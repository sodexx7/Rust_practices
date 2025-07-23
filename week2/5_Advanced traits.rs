use std::{fmt::Display};
use std::ops::{Add, Sub,Mul,Div,BitAnd,BitOr,BitXor};
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


#[derive(Debug)]
 struct Calculator<T> {
    x: T,
    y: T,
 }


 trait AdditiveOperations<T> {
    fn addition(&self) -> T;
    fn substraction(&self) -> T;
}

trait MultiplicativeOperations<T> {
    fn multiplication(&self) -> T;
    fn division(&self) -> T;

}

trait BinaryOperations<T> {
    fn and(&self) -> T;
    fn or(&self) -> T;
    fn xor(&self) -> T;
}

impl <T> AdditiveOperations<T> for Calculator<T>
where T: Add<Output = T> + Sub<Output = T> + Copy,
{

    fn addition(&self) -> T {
       self.x + self.y
    }
   

    fn substraction(&self) -> T {
        self.x - self.y
    }
}

impl <T> MultiplicativeOperations<T> for Calculator<T> 
where T:Mul<Output = T> + Div<Output = T> + Copy
{
    fn multiplication(&self) -> T {
        self.x * self.y
    }

    fn division(&self) -> T {
        self.x / self.y
    }
}

impl <T> BinaryOperations<T> for Calculator<T>
where T:BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> +  Copy
 {
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


fn print_output<T:Display>(calculator:&(impl AdditiveOperations<T> + MultiplicativeOperations<T> + BinaryOperations<T>)) 
{
    let mut operations = format!("addition operation {}",calculator.addition());
    operations.push_str(&format!("\n substraction operation {}",calculator.substraction()));
    operations.push_str(&format!("\n multiplication operation {}",calculator.multiplication()));
    operations.push_str(&format!("\n division operation {}",calculator.division()));
    operations.push_str(&format!("\n and operation {}",calculator.and()));
    operations.push_str(&format!("\n or operation {}",calculator.or()));
    operations.push_str(&format!("\n xor operation {}",calculator.xor()));
    println!("{operations}");
}


impl <T> Display for Calculator<T> 
where T:Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",format!("(x={}, y={})", self.x, self.y))

    }
 }


 fn main(){

    println!("{}",i32::MAX);
    let calculator: Calculator<i32> = Calculator {x:100,y:200};
    println!("calculator: {}", calculator); 
    println!("Apply print_output....................................");
    print_output(&calculator);

 }