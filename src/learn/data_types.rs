use colored::*;

pub fn data_types() -> String {
    println!("\n{}", "Data Types".green().bold());

    // Variable type defines what to parse to
    let x: u32 = "42".parse().expect("Not a number!");
    println!("The value of x is: {}", x);

    // Integer literals
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte = b'A'; // (u8 only)

    // System integer types
    let _x: isize = 10;
    let _x: usize = 10;

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    // Arrays
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let a = serde_json::to_string(&a).expect("Failed to serialize");
    println!("The value of a is: {}", a);

    a
}
