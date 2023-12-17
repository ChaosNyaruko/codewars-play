use codewars::{
    decompose::{self, dfs},
    move_zeros::{self},
    sudoku::sudoku,
};
use std::collections::HashMap;

fn find_smallest_int(arr: &[i32]) -> i32 {
    // your code here
    let mut res: i32 = i32::MAX;
    for &x in arr {
        if x < res {
            res = x
        }
    }
    res
}

fn hello(name: &str) -> String {
    if name.is_empty() {
        return String::from("Hello, World!");
    }
    return format!(
        "Hello, {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    );
}

fn rps(p1: &str, p2: &str) -> &'static str {
    let m = |s| match s {
        "scissors" => 0,
        "paper" => 1,
        "rock" => 2,
        _ => panic!("unexpected input"),
    };
    let a = m(p1);
    let b = m(p2);

    if a == b {
        "Draw!"
    } else if b == (a + 1) % 3 {
        "Player 1 won!"
    } else {
        "Player 2 won!"
    }
}

fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => false,
            _ => true,
        })
        .collect()
}

fn main() {
    let mut memo: HashMap<(i64, i64), Option<Vec<i64>>> = HashMap::new();
    if let Some(m) = dfs(87, 44, &mut memo) {
        eprintln!("{m:?}");
    } else {
        panic!("whatsoever");
    }
    let mut puzzle = [
        [0, 0, 8, 0, 3, 0, 5, 4, 0],
        [3, 0, 0, 4, 0, 7, 9, 0, 0],
        [4, 1, 0, 0, 0, 8, 0, 0, 2],
        [0, 4, 3, 5, 0, 2, 0, 6, 0],
        [5, 0, 0, 0, 0, 0, 0, 0, 8],
        [0, 6, 0, 3, 0, 9, 4, 1, 0],
        [1, 0, 0, 8, 0, 0, 0, 2, 7],
        [0, 0, 5, 6, 0, 3, 0, 0, 4],
        [0, 2, 9, 0, 7, 0, 8, 0, 0],
    ];
    let solution = [
        [9, 7, 8, 2, 3, 1, 5, 4, 6],
        [3, 5, 2, 4, 6, 7, 9, 8, 1],
        [4, 1, 6, 9, 5, 8, 3, 7, 2],
        [8, 4, 3, 5, 1, 2, 7, 6, 9],
        [5, 9, 1, 7, 4, 6, 2, 3, 8],
        [2, 6, 7, 3, 8, 9, 4, 1, 5],
        [1, 3, 4, 8, 9, 5, 6, 2, 7],
        [7, 8, 5, 6, 2, 3, 1, 9, 4],
        [6, 2, 9, 1, 7, 4, 8, 5, 3],
    ];

    sudoku(&mut puzzle);
    assert_eq!(
        puzzle, solution,
        "\nYour solution (left) did not match the correct solution (right)"
    );
}
