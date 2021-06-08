use std::cmp;

#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    if len < 1 {
        return String::new();
    }

    if s == s.chars().rev().collect::<String>() {
        return s;
    }

    let mut start: usize = 0;
    let mut end: usize = 0;

    for i in 0..len {
        let len1 = expand(&s, i, i);
        let len2 = expand(&s, i, i + 1);
        let len = cmp::max(len1, len2);
        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    s[start..end + 1].to_string()
}

fn expand(s: &str, left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;
    while l >= 0
        && r < s.len() as i32
        && s.chars().nth(l as usize).unwrap() == s.chars().nth(r as usize).unwrap()
    {
        r += 1;
        l -= 1;
    }

    (r - l - 1) as usize
}

#[test]
fn it_should_run_correctly() {
    let r1 = longest_palindrome("babad".to_string());
    assert_eq!("aba", r1);
    let r2 = longest_palindrome("cbbd".to_string());
    assert_eq!("bb", r2);
    let r3 = longest_palindrome("a".to_string());
    assert_eq!("a", r3);
    let r4 = longest_palindrome("ac".to_string());
    assert_eq!("c", r4);
}
