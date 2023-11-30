use codewars::{move_zeros::{self}, decompose::{self, dfs}};
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
}
