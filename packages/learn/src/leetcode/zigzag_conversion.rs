#[allow(dead_code)]

pub fn convert(s: String, num_rows: i32) -> String {
    let mut d: Vec<Vec<char>> = std::iter::repeat(vec![]).take(num_rows as usize).collect::<Vec<_>>();

    if num_rows == 1 {
        return s;
    }

    let mut dir: i32 = 1;
    let mut step: i32 = 0;

    for c in s.chars() {
        d[step as usize].push(c);

        if step == 0 {
            dir = 1;
        }

        if step >= num_rows - 1 {
            dir = -1;
        }

        step += dir;
    }

    d.concat().iter().collect::<String>()
}

#[test]
fn it_should_run_correctly() {
    let r1 = convert("PAYPALISHIRING".to_string(), 3);
    assert_eq!("PAHNAPLSIIGYIR", r1);
    let r1 = convert("AB".to_string(), 1);
    assert_eq!("AB", r1);
}
