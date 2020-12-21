use colored::*;

pub fn strings() {
    println!("\n{}", "Strings".green().bold());

    let mut s = String::new();
    println!("new string: {}", s);

    let data = "initial contents こんにちは";
    s = data.to_string();
    println!("string: {}", s);

    s = String::from("initial contents 2 こんにちは");
    println!("string: {}", s);

    s.push_str(" added");
    println!("string: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("combine: {}", s3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let tictac = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", tictac);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
