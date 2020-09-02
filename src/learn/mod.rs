#[allow(dead_code)]
mod control;
#[allow(dead_code)]
mod data_types;
#[allow(dead_code)]
mod guess;
#[allow(dead_code)]
mod ownership;
#[allow(dead_code)]
mod variables;
#[allow(dead_code)]
mod references;
#[allow(dead_code)]
mod slice;
#[allow(dead_code)]
mod structs;
#[allow(dead_code)]
mod enums;

pub fn run() {
    guess::high_and_low_example();
    // guess::guess_number();
    variables::variables();
    data_types::data_types();
    control::control();
    ownership::ownership();
    references::references();
    slice::slice();
    structs::structs();
    enums::enums();
}
