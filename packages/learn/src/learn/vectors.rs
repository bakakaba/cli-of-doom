use colored::*;

pub fn vectors() {
    println!("\n{}", "Vectors".green().bold());

    let v: Vec<i32> = Vec::new();
    println!("The value of v is: {:#?}", v);

    let v2 = vec![1, 2, 3];
    println!("The value of v2 is: {:?}", v2);

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("The value of v3 is: {:?}", v3);

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // This throws an index out of bounds
    let does_not_exist = v.get(100);
    println!("Missing element: {:?}", does_not_exist);

    for i in &mut v3 {
        *i += 50;
    }

    println!("The value of v3 is now: {:?}", v3);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
