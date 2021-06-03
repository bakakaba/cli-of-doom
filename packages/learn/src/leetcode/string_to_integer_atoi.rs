#![allow(dead_code)]

pub fn my_atoi(s: String) -> i32 {
    let mut num = String::new();
    let mut symbol_added = false;
    let mut number_started = false;

    for c in s.chars() {
        if c == '-' || c == '+' {
            if symbol_added || number_started {
                break;
            }
            symbol_added = true;
            num.push(c);
            continue;
        }

        if c == ' ' {
            if number_started || symbol_added {
                break;
            }
            continue;
        }

        if c.is_numeric() {
            number_started = true;
            num.push(c);
            continue;
        }

        break;
    }

    let len = num.len() - (if symbol_added { 1 } else { 0 });
    if len == 0 {
        return 0;
    }

    let val = num.parse::<i32>();
    match val {
        Ok(x) => return x,
        Err(_) => {
            if num.starts_with('-') {
                return std::i32::MIN;
            } else {
                return std::i32::MAX;
            }
        }
    }
}

#[test]
fn it_should_run_correctly() {
    let r1 = my_atoi("42".to_string());
    assert_eq!(42, r1);
    let r2 = my_atoi("   -42".to_string());
    assert_eq!(-42, r2);
    let r3 = my_atoi("4193 with words".to_string());
    assert_eq!(4193, r3);
    let r4 = my_atoi("words and 987".to_string());
    assert_eq!(0, r4);
    let r5 = my_atoi("-91283472332".to_string());
    assert_eq!(-2147483648, r5);
    let r6 = my_atoi("-+12".to_string());
    assert_eq!(0, r6);
    let r7 = my_atoi("+1".to_string());
    assert_eq!(1, r7);
    let r8 = my_atoi("+".to_string());
    assert_eq!(0, r8);
    let r9 = my_atoi("   +0 123".to_string());
    assert_eq!(0, r9);
    let r10 = my_atoi("-5-".to_string());
    assert_eq!(-5, r10);
    let r11 = my_atoi("  +  413".to_string());
    assert_eq!(0, r11);
}
