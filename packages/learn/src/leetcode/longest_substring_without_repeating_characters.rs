#![allow(dead_code)]

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let mut t = 0;

    for (pos, c) in s.chars().enumerate() {
        while s[t..pos].chars().any(|x| x == c) {
            t += 1;
        }
        // println!("pos: {} t: {}", pos, t);
        let len = pos - t + 1;
        if  len > longest {
            longest = len;
        }
    }

    longest as i32
}

#[test]
fn it_should_run_correctly() {
    let r1 = length_of_longest_substring("abcabcbb".to_string());
    assert_eq!(3, r1);
    let r2 = length_of_longest_substring("bbbbb".to_string());
    assert_eq!(1, r2);
    let r3 = length_of_longest_substring("pwwkew".to_string());
    assert_eq!(3, r3);
    let r4 = length_of_longest_substring("".to_string());
    assert_eq!(0, r4);
}
