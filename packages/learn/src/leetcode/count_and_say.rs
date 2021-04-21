#[allow(dead_code)]
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return String::from("1");
    }

    let result = count_and_say(n - 1) + "$";

    let mut count = 0;
    let mut prev: char = '^';
    let mut out = String::new();
    for c in result.chars() {
        if prev == c {
            count += 1;
        } else {
            if count > 0 {
                let mut s =count.to_string();
                s.push(prev);
                out += &s;
            }
            prev = c;
            count = 1;
        }
    }

    out
}

#[test]
fn it_count_and_say() {
    let result = count_and_say(4);
    println!("{:?}", result);
    assert_eq!("1211", result);
}
