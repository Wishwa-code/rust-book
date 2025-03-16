/**
 * * Shadowing
 *
 * ? There's one more way to change the value of variables that were defined using `let`. This is called shadowing.
 * ? Basically, what happens is we define another variable with the same name without considering the type of the original variable.
 * ? The new value is assigned, and once the code block ends, the last value that was assigned using the `let` variable will be the value of the `let` variable.
 * ? This is useful when we need to change the type of the variable without defining a new variable.
 */

fn main() {
let x = 5;

let x = x + 1;
{
    let x= x * 2;
    println!("the values of the x in the inner scoper is: {x}");
}

println!("The value of x is: {x}");

}



/**
* *Variables and mutability
* ? by dfault all the variables in rust are immutable and if you want to make a variable 
* ? mutable you need to add mut keyword infront of let keyword this is mostly likely to be used in most scenerios
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x= 6;
   println!("The value of x is: {x}");
}
*/

/**
* * Constants
* ? constans value are constand no matter what happens and capital letters and underscores need to
* ? be used to name constant variables and the value should not depend on a value that is generated in 
* ? in runtime. Can be used to define harcoded values mostly
* ? also, you need tto explictly define the type of the variable when using const
const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
*/