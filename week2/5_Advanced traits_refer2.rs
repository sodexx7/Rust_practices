// when apply generic type. impl the related struct should specify the T related with the traits
// refer_2  Integer represent all usable traits(Add, BitAnd, BitOr, BitXor, Div, Mul, Sub), then reference generic T as Integer
// refer_1  specify the generic type T along with the needed traits when impling each trait.  
use std::{fmt::Display, ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Sub}};

// TRAITS //////////////////////////////////////
trait Integer: Display 
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Copy {}


impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}

trait AdditiveOperations<T: Integer> {
    fn addition(&self) -> T;

    fn subtraction(&self) -> T;
}

trait MultiplicativeOperations<T: Integer> {
    fn multiply(&self) -> T;

    fn divide(&self) -> T;
}

trait BinaryOperations<T: Integer> {
    fn and(&self) -> T;

    fn or(&self) -> T;

    fn xor(&self) -> T;
}

// STRUCTS //////////////////////////////////////
struct Calculator<T: Integer> {
    left: T,
    right: T,
}

// // Implement traits for the Calculator structure
// impl<T> AdditiveOperations<T> for Calculator<T>
// where
//     T: Add<Output = T> + Sub<Output = T> + Copy,
// {
//     fn add(&self) -> T {
//         self.value + self.value2
//     }
//     fn subtract(&self) -> T {
//         self.value - self.value2
//     }
// }

impl<T: Integer> AdditiveOperations<T> for Calculator<T> {
    fn addition(&self) -> T {
        self.left + self.right
    }

    fn subtraction(&self) -> T {
        self.left - self.right
    }
}
impl<T: Integer> MultiplicativeOperations<T> for Calculator<T> {
    fn multiply(&self) -> T {
        self.left * self.right
    }

    fn divide(&self) -> T {
        self.left / self.right
    }
}
impl<T: Integer> BinaryOperations<T> for Calculator<T> {
    fn and(&self) -> T {
        self.left & self.right
    }

    fn or(&self) -> T {
        self.left | self.right
    }

    fn xor(&self) -> T {
        self.left ^ self.right
    }
}

impl<T: Integer> Display for Calculator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Calculator has left: {} and right: {}", self.left, self.right)?;
        writeln!(f, "Addition: {} + {} = {}", self.left, self.right, self.addition())?;
        writeln!(f, "Subtraction: {} - {} = {}", self.left, self.right, self.subtraction())?;
        writeln!(f, "Multiplication: {} * {} = {}", self.left, self.right, self.multiply())?;
        writeln!(f, "Division: {} / {} = {}", self.left, self.right, self.divide())?;
        writeln!(f, "AND: {} & {} = {}", self.left, self.right, self.and())?;
        writeln!(f, "OR: {} | {} = {}", self.left, self.right, self.or())?;
        writeln!(f, "XOR: {} ^ {} = {}", self.left, self.right, self.xor())
    }
}

// MAIN //////////////////////////////////////
fn main() {

    let c: Calculator<u8> = Calculator { left: 3, right: 2 };
    println!("{}", c);

    println!();

    let c: Calculator<i8> = Calculator { left: 2, right: 3 };
    println!("{}", c)
}