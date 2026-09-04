fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Declaring Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // SHADOWING
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    let x = x * 2;
    println!("The value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Space length is : {spaces}")

    // let mut spaces = "       ";
    // spaces = spaces.len(); // here mut not changed data type.
    // println!("Space length is : {spaces}") 
    
}