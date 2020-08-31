pub mod guess;

pub fn _run() {
    let result = guess::_high_and_low("10 20 40 50 75");
    println!("{}", result);

    guess::_guess_number()
}