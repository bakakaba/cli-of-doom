use super::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
fn panic_test() {
    panic!("Make this test fail");
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[test]
#[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
fn greater_than_100() {
    Guess::new(200);
}

#[test]
fn it_works2() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }}
