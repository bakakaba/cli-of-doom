#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn partition(arr: &mut Vec<i32>, side: i32, idx: i32, sum: i32, count: i32) -> bool {
        if count == 3 {
            return true;
        }
        if sum == side {
            return Solution::partition(arr, side, 0, 0, count + 1);
        }
        if sum > side {
            return false;
        }

        for i in idx as usize..arr.len() {
            if arr[i] == 0 {
                continue;
            }

            let num = arr[i];
            arr[i] = 0;

            if Solution::partition(arr, side, (i + 1) as i32, sum + num, count) {
                return true;
            }

            arr[i] = num;
        }

        false
    }

    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let mut arr = matchsticks.clone();
        let total = arr.iter().fold(0, |a, x| a + x);
        if total == 0 || total % 4 != 0 {
            return false;
        }
        let side = total / 4;

        return Solution::partition(&mut arr, side, 0, 0, 0);
    }
}

#[test]
fn it_makes_a_square() {
    let result = Solution::makesquare(vec![1, 1, 2, 2, 2]);
    assert!(result);
}

#[test]
fn it_forms_a_square() {
    let result = Solution::makesquare(vec![10, 6, 5, 5, 5, 3, 3, 3, 2, 2, 2, 2]);
    assert!(result);
}

#[test]
fn it_also_forms_a_square() {
    let result = Solution::makesquare(vec![2, 2, 2, 3, 4, 4, 4, 5, 6]);
    assert!(result);
}
