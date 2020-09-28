use dotenv::dotenv;

mod learn;

fn main() {
    dotenv().ok();
    learn::run();
}
