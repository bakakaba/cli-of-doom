use cli_of_doom::learn::testing;
mod common;

#[test]
fn it_adds_two_integration() {
    common::setup();
    assert_eq!(4, testing::add_two(2));
}
