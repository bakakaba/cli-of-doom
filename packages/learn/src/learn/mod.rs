mod control;
mod data_types;
mod enums;
mod errors;
mod generics;
mod guess;
mod hashmap;
mod lifetimes;
mod ownership;
mod references;
mod slice;
mod strings;
mod structs;
mod traits;
mod variables;
mod vectors;

pub mod documentation;
pub mod functional;
pub mod testing;

pub fn run() {
    // guess::guess_number();

    guess::high_and_low_example();

    variables::variables();
    data_types::data_types();
    control::control();
    ownership::ownership();
    references::references();
    slice::slice();
    structs::structs();
    enums::enums();
    vectors::vectors();
    strings::strings();
    hashmap::hashmap();
    errors::errors();
    generics::generics();
    traits::traits();
    lifetimes::lifetimes();
    testing::testing();

    functional::closures();
    functional::iterators();
}
