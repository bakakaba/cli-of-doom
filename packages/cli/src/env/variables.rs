use colored::Colorize;

pub fn list_variables() {
    for (key, value) in std::env::vars() {
        println!("{}: {}", key.green(), value);
    }
}
