use colored::*;

pub fn slice() {
    println!("\n{}", "Slice".green().bold());

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word={}", word);
}
