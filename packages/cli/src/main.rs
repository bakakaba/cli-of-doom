#[allow(unused_imports)]
use learn::learn;
use dotenv::dotenv;

// mod grep;

fn main() {
    dotenv().ok();

    learn::run();
    // grep::run();
}
