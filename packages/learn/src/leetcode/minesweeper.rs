use std::{char, usize};

struct Solution {}

impl Solution {
    fn count_mines(board: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let max_y = board.len() as i32;
        let max_x = board[0].len() as i32;
        let mut mines = 0;

        for d_x in -1..=1 {
            for d_y in -1..=1 {
                let n_x = x + d_x;
                let n_y = y + d_y;

                if n_x >= 0 && n_x < max_x && n_y >= 0 && n_y < max_y {
                    if board[n_y as usize][n_x as usize] == 'M' {
                        mines += 1;
                    }
                }
            }
        }

        mines
    }

    fn check_point(board: &mut Vec<Vec<char>>, x: i32, y: i32) {
        if board[y as usize][x as usize] != 'E' {
            return;
        }

        let max_y = board.len() as i32;
        let max_x = board[0].len() as i32;

        let mines = Solution::count_mines(board, x, y);
        if mines > 0 {
            board[y as usize][x as usize] = char::from_digit(mines as u32, 10).unwrap();
            return;
        } else {
            board[y as usize][x as usize] = 'B';
        }

        for d_x in -1..=1 {
            for d_y in -1..=1 {
                let n_x = x + d_x;
                let n_y = y + d_y;

                if n_x >= 0 && n_x < max_x && n_y >= 0 && n_y < max_y {
                    Solution::check_point(board, n_x, n_y);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut b = board.clone();
        let y = click[0];
        let x = click[1];

        if board[y as usize][x as usize] == 'M' {
            b[y as usize][x as usize] = 'X';
        } else {
            Solution::check_point(&mut b, x, y);
        }

        b
    }
}

#[test]
fn it_should_update() {
    let board = Solution::update_board(
        vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'M', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ],
        vec![3, 0],
    );
    assert_eq!(
        vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B']
        ],
        board
    );
}

#[test]
fn it_should_explode() {
    let board = Solution::update_board(
        vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'M', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ],
        vec![1, 2],
    );
    assert_eq!(
        vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'X', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ],
        board
    );
}

#[test]
fn it_failed_this_case() {
    let board = Solution::update_board(
        vec![
            vec!['E', 'E', 'E', 'E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E', 'E', 'E', 'M'],
            vec!['E', 'E', 'M', 'E', 'E', 'E', 'E', 'E'],
            vec!['M', 'E', 'E', 'E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'M', 'M', 'E', 'E', 'E', 'E'],
        ],
        vec![0, 0],
    );

    assert_eq!(
        vec![
            vec!['B', 'B', 'B', 'B', 'B', 'B', '1', 'E'],
            vec!['B', '1', '1', '1', 'B', 'B', '1', 'M'],
            vec!['1', '2', 'M', '1', 'B', 'B', '1', '1'],
            vec!['M', '2', '1', '1', 'B', 'B', 'B', 'B'],
            vec!['1', '1', 'B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', '1', '2', '2', '1', 'B', 'B', 'B'],
            vec!['B', '1', 'M', 'M', '1', 'B', 'B', 'B']
        ],
        board
    );
}
