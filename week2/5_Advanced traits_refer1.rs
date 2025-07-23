use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor};

// when apply generic type. impl the related struct should specify the T related with the traits

// Define traits for operations
trait AdditiveOperations<T> {
    fn add(&self) -> T;
    fn subtract(&self) -> T;
}

trait MultiplicativeOperations<T> {
    fn multiply(&self) -> T;
    fn divide(&self) -> T;
}

trait BinaryOperations<T: BitOr + BitXor + BitAnd> {
    fn bitwise_and(&self) -> T;
    fn bitwise_or(&self) -> T;
    fn bitwise_xor(&self) -> T;
}
// Define the Calculator structure
struct Calculator<T> {
    value: T,
    value2: T,
}
// Implement traits for the Calculator structure
impl<T> AdditiveOperations<T> for Calculator<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    fn add(&self) -> T {
        self.value + self.value2
    }
    fn subtract(&self) -> T {
        self.value - self.value2
    }
}
impl<T> MultiplicativeOperations<T> for Calculator<T>
where
    T: Mul<Output = T> + Div<Output = T> + Copy,
{
    fn multiply(&self) -> T {
        self.value * self.value2
    }
    fn divide(&self) -> T {
        self.value / self.value2
    }
}
// Implement the Display trait for the Calculator struct
impl<T> Display for Calculator<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addition = self.add();
        let subtraction = self.subtract();
        let multiplication = self.multiply();
        let division = self.divide();
        
        write!(
            f,
            "
            Addition: {}
            Subtraction: {}
            Multiplication: {}
            Division: {}",
            addition, subtraction, multiplication, division
        )
        
    }
}

// Define the Calculator structure
struct BinaryCalculator<T> {
    calculator: Calculator<T>
}

// Implement the trait for BinaryCalculator
impl<T> BinaryOperations<T> for BinaryCalculator<T>
where
    T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy,
{
    fn bitwise_and(&self) -> T {
        self.calculator.value & self.calculator.value2
    }
    fn bitwise_or(&self) -> T {
        self.calculator.value | self.calculator.value2
    }
    fn bitwise_xor(&self) -> T {
        self.calculator.value ^ self.calculator.value2
    }
}
// Implement AdditiveOperations for BinaryCalculator by delegating to Calculator
impl<T> AdditiveOperations<T> for BinaryCalculator<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    fn add(&self) -> T {
        self.calculator.add()
    }

    fn subtract(&self) -> T {
        self.calculator.subtract()
    }
}

// Implement MultiplicativeOperations for BinaryCalculator by delegating to Calculator
impl<T> MultiplicativeOperations<T> for BinaryCalculator<T>
where
    T: Mul<Output = T> + Div<Output = T> + Copy,
{
    fn multiply(&self) -> T {
        self.calculator.multiply()
    }
    fn divide(&self) -> T {
        self.calculator.divide()
    }
}

// Implement the Display trait for the Calculator struct
impl<T> Display for BinaryCalculator<T>
where T: BitAnd<Output = T>
    + BitOr<Output = T>
    + BitXor<Output = T>
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + Copy
    + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addition = self.add();
        let subtraction = self.subtract();
        let multiplication = self.multiply();
        let division = self.divide();
        let bitwise_and = self.bitwise_and();
        let bitwise_or = self.bitwise_or();
        let bitwise_xor = self.bitwise_xor();
        write!(
                f,
                "
                Addition: {}
                Subtraction: {}
                Multiplication: {}
                Division: {}
                Bitwise AND: {}
                Bitwise OR: {}
                Bitwise XOR: {}",
                addition, subtraction, multiplication, division, bitwise_and, bitwise_or, bitwise_xor
            )
    }
}
// Main function to demonstrate the operations
fn main() {
    // Integer example
    let int_calc = BinaryCalculator{ calculator: Calculator { value: 10, value2: 2 } };
    println!("Implem = {}", int_calc);
    // Float example
    let float_calc = Calculator { value: 10.0, value2: 2.0 };
    println!("Implem = {}", float_calc);
}