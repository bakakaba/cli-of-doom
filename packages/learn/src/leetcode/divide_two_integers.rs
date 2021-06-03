#![allow(dead_code)]

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    dividend.wrapping_div(divisor)
}

#[test]
fn it_should_run_correctly() {
    let r1 = divide(8, 4);
    assert_eq!(2, r1);
    // let r2 = divide(-2147483648, -1);
    // assert_eq!(2147483647, r2);
}
