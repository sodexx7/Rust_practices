

/** Ownership and Borrowing)
 * 
 * 
 */

fn main() {
    // let variable = String::from("Welcome to RustSkills");
    // replace the String variable with a scalar variable (u32, i32, u64, i64, …) Why does it work?
    
    /**
     *  scale variable implement Copy. instead of transferring ownership. the var function_1 copy variable. As the scale variable is cheap to copy and only store in the stack
     * 
     * 
     */
    let variable:u32 = 56;

    function_1(variable);

    println!("In main, variable is: {}", variable);
}


fn function_1(var: u32) {

    println!("In function_1, variable is: {}", var);
}


/**
 * - Explain why this code snippet does not work.
 *      1.  function_1(variable); has got variable's ownership, 
 *          then end of function_1 that will drop of the variable. so in the main scope, don't own the variable,so  the println can't use variable.
 *   Give at least 2 ways to fix the issue (there are more than 2).
 * 
 * 
 */


fn _string_Test() {
    let variable: String = String::from("Welcome to RustSkills");


    // solution 4. clone variable
    // let new_variable: String = variable.clone();
    // function_1(variable);
    // solution_1(&variable);



    // solution2: use inner scope dealing with the variable. 
    // {   
    //     println!("In function_1, variable is: {}", variable);
    // }

    // let variable:String= solution_3(variable);

    println!("In main, variable is: {}", variable);
}

// fn function_1(var: String) {

//     println!("In function_1, variable is: {}", var);
// }

// solution_1 borrow 
fn solution_1(var:&String){
    println!("In function_1, variable is: {}", var);
 
}

// solution3 return ownership to main scope
fn solution_3(var:String) -> (String) {
    println!("In function_1, variable is: {}", var);
    return var;

}

