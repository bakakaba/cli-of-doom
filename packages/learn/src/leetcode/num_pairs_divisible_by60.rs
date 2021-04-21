#[allow(dead_code)]
pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut remainders = [0; 60];
    let mut count = 0;

    for t in time {
        let r = t % 60;
        count += remainders[((60 - r) % 60) as usize];
        remainders[r as usize] += 1;
    }

    count
}

#[test]
fn it_num_pairs_divisible_by60() {
    let result = num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]);
    println!("{:?}", result);
    assert_eq!(3, result);
}
