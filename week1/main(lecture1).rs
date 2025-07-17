fn main(){
    let mut  v:Vec<i32> = vec![1,2];
    take(&mut v); // main{v,take(arg(&v))} 
    
    // main(v)
    println!("{:?}",v);
}
fn take(arg:&mut Vec<i32>) {
    arg.push(99);
    println!("{:?}",arg); // main(arg)

}


// // A borrow & keeps ownership in the original scope, but allows temporaray success.
// // u8, u16 -> these all implement Copy , and so have copy semantic 
// fn main(){
//     let v:Vec<i32> = vec![1,2];
//     // main(v)
//     take(v); // main{v,take(arg(&v))} 
    
//     // main(v)
//     println!("{:?}",v)
// }
// fn take(arg:u8) {
//     // take{arg}
//     println!("{:?}",arg); // main(arg)

// }

// fn take(arg:&Vec<i32>) {
//     // take{arg}
//     println!("{:?}",arg); // main(arg)

// } // drop all owned data, arg gets dropped (which was not v as it is only a borrow)


// // No & mean its a move.
// fn main(){
//     let v:Vec<i32> = vec![1,2];
//     // main(v)
//     take(v); // main{take(arg(v))} 
//     println!("{:?}",v)
// }

// fn take(arg:Vec<i32>) {
//     // take{arg}
//     println!("{:?}",arg); // main(arg)

// } // drop all owned data, arg gets dropped (which was v from other scope)


/** OWNERSHIPS (simplify things to lexicial lifetimes)
 *  All data is owned by exactly 1 variable, handle.
 *  By owning data, a var controlls the read and write access to that data.
 *  All vars that own data care contained in some scope. Scope being the area inside crulys {}
 *  Ownership can be transferred with a move to another var.
 *  At the end of a scope we drop all owned data
 */
fn _scopes(){

    let x = vec![1,2]; // x owns the data on the stack, and on the heap.
    // main(x)

    let y = x;
    // main(y)

    println!("{:?}",y); // main(y)
    // println!("{}",x); // main(y)
    
    let x: Vec<i32> = vec![3,4];
    // main(y,x)
    {
        // main(y,x,block{})
        println!("INNER {:?}",x); 
        let x: Vec<_> = vec![5,6];
        // main(y,x block{x})
        println!("{:?}",x);
        // main(y,x) - DROP ALL OWNEED DATA BY THE SCOPE  (inner x)
    }
    // main(y,x)
    println!("{:?}",x);
    println!("{:?}",y);


}  // nothing left, everything drop



/**STRING and strs
 * str ALWAYS ALLOCATED in static memory, and you can only access through a reference(borrow,raw pointer,a Box)
 * - immutable and always fixed size. Linda like an immutable array of chars.
 * String is literally a Vec<u8> where u8 is intepretted as a char.
 * - Variable size, mutable
 */

 fn _strs(){
    // let s:&str = 'asasdasd'; // Not allowed , need the reference
    let s:&str = "asasdasd"; // quesiton cal modify this s?


    let mut s2: String = String::from("Hello");
    s2 = s2 + s;
    println!("{}",s2);

 }




/** AGGREGATE
 * 
 */

 fn _agg() {

    // HEAP BOX
    // let x:Box<I32> = Box::new(5);
    //  ^x = 0x123123.        ^5 is stored in      0x123123

    // TUPLES
    // Stack, fixed size. hetrogeneous lists
    // let tup: (i32,u8,&str) = (-1, 255, "hello");
    let tup: (i32,u8,Box<&str>) = (-1, 255, Box::new("hello"));
    println!("{:?}",tup);
    println!("{:?}",tup.2);

    // ARRAYS
    // Stack fixed size, homogenerous lists.
    let mut arr:[u8;5] = [1,2,3,4,5];
    println!("{:?}",arr);
    println!("{:?}",arr[2]);

    // SLICE reference to aggregate data
    let slice = &mut arr[0..=3];
    slice[0] = 123;
    println!("{:?}",slice);
    println!("{:?}",arr);

    // VECTORS
    // Stack pointers to the heap. variable size, homogenerous list.
    let mut vec:Vec<i8> = vec![-1,0,1];
    println!("{:?} cap = {}",vec,vec.capacity());
    vec.push(11);
    println!("{:?} cap = {}",vec,vec.capacity());
    vec.pop();
    vec.pop(); // after two pop, no affecting the capacity.
    println!("{:?} cap = {}",vec,vec.capacity());
    // println!("{:?}",vec[1]);






 }



/** MUTABILITY
 *  All data is immutable by default (different to a constant though)
 *  MUTABILITY is a property of the building, bot the underlying data.
 * 
 */

fn _mut () {
    // const X :i32 = 44; // not the same as immutable var
    // let x : i32 = 44;
    // let x = 45 ;
    

    let mut y:String = String::from("hello");
    y.push_str("World");
    print!("{}",y);

    let z:String = y;
    // z.push_str("World");  // NOT Allowed, z was not delared mut
}




/** SCALES AND BOOLS
 * UNSIGNED INTS = u8,u16,u32,u54,u128. (!u256) usize -> ? u64 ==> 64 bits wordsize, usize == u32 ==> 32 bits wordsize
 *  SIGNED INTS =  i8,i16,i32,i54,i128. (!u256) isize -> ? u64 ==> 64 bits wordsize, usize == u32 ==> 32 bits wordsize
 * 
 */

fn _ints() {

    // let x : u8 = u8::MAX +1; // Not permissible``
    let x: u16 = 259;
    let x = 45 ; // shadowing
    _take(x as u8);
    println!("{}",x);

     use ethnum::U256;
    // U256 
    let y:u128 = u128::MAX;
    let z:U256 = U256::MAX;
    println!("{}",y);
    println!("{}",z);


    
}
fn _take(arg:u8){
    _ = arg;
    println!("{}",arg)

}

fn return_something() -> Option<u64> {
    Some(12345)
}

fn return_nothing() -> Option<u64> {
    None
}

fn print_output(option: Option<u64>) {
    match option {
        Some(value) => println!("There is a value and it is: {}", value),
        None => println!("There is no value")
    }
}

// fn main() {
//     print_output(return_nothing()); // Option will be None

//     print_output(return_something()); // Option will be Some(u64)

//     println!("\n--- Stack vs. Heap Demo ---");
//     stack_and_heap_demo();
// }

fn stack_and_heap_demo() {
    // --- Stack Allocation ---

    // `x` is an integer. Its size is known at compile time (4 bytes for i32).
    // It is stored directly on the stack.
    let x = 42; 

    // `y` is a reference to `x`. The reference itself is stored on the stack.
    // It holds the memory address of `x`.
    let y = &x;

    println!("--- Stack ---");
    println!("x (value): {} is stored on the stack.", x);
    println!("y (a reference): {:p} is on the stack, pointing to x's address.", y);


    // --- Heap Allocation ---

    // `b` is a `Box<i32>`. A Box is a "smart pointer" that allocates memory on the heap.
    // The actual value `5` is stored on the heap.
    // The pointer to that data (`b`) is stored on the stack.
    let b = Box::new(5);

    // `s` is a String. Its size can change (it's growable).
    // The actual text content ("hello") is stored on the heap.
    // The pointer to the text, its length, and its capacity are stored on the stack.
    let s = String::from("hello");

    println!("\n--- Heap ---");
    println!("b (value): {} is stored on the heap.", b);
    println!("s (value): \"{}\" is stored on the heap.", s);
    
    // When this function ends, `x`, `y`, `b`, and `s` go out of scope.
    // Rust automatically cleans up the stack memory for `x` and `y`.
    // For `b` and `s`, it calls `drop`, which deallocates the memory they point to on the heap.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // First unit test
    fn nothing_unit_test() {
        assert_eq!(return_nothing(), None);
    }

    #[test] // Second unit test
    fn something_unit_test() {
        assert_eq!(return_something(), Some(12345));
    }

    #[test] // Third unit test
    fn something2_unit_test() {
        helper_checker(return_something(), 999).unwrap(); // will panic if not the same values, unit test will fail
    }
    
    fn helper_checker(value: Option<u64>, expected: u64) -> Result<(), ()> {
        let int_value = value.expect("value is an Error, it doesn't contain an Ok(u64)"); // Will panic here if `value` result is an Error
        if int_value == expected {
            Ok(())  // return Ok if same value
        } else {
            Err(()) // else return an error
        }
    }

}