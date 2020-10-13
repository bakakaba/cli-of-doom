use dotenv::dotenv;
use cli_of_doom::learn;

fn main() {
    dotenv().ok();
    learn::run();
}
