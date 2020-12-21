use colored::*;

pub fn ownership() {
    println!("\n{}", "Ownership".green().bold());

    // Stack is copied, heap is moved

    // Deep clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Function ownership is the same as assignment
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    let s = String::from("hello");
    let s = takes_and_gives_back(s);
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();

        (s, length)
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
