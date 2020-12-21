use colored::*;

pub fn variables() {
    println!("\n{}", "Variables".green().bold());

    // Mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000; // Underscore in the number is optional
    println!("Constant number: {}", MAX_POINTS);

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);
}
