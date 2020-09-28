#[allow(dead_code)]
mod control;
#[allow(dead_code)]
mod data_types;
#[allow(dead_code)]
mod enums;
#[allow(dead_code)]
mod guess;
#[allow(dead_code)]
mod hashmap;
#[allow(dead_code)]
mod ownership;
#[allow(dead_code)]
mod references;
#[allow(dead_code)]
mod slice;
#[allow(dead_code)]
mod strings;
#[allow(dead_code)]
mod structs;
#[allow(dead_code)]
mod variables;
#[allow(dead_code)]
mod vectors;

pub fn run() {
    // guess::guess_number();
    // panic!("crash and burn");

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
}
