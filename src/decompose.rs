use std::collections::HashMap;

pub fn dfs(
    n: i64,
    last: i64,
    memo: &mut HashMap<(i64, i64), Option<Vec<i64>>>,
) -> Option<Vec<i64>> {
    // eprintln!("calulating: {n}, {last}");
    // assert!(n >= 0);
    if n == 0 {
        return Some(vec![]);
    }
    if n == 1 && last <= 1 {
        return None;
    }
    if n == 1 {
        return Some(vec![1]);
    }
    if let Some(x) = memo.get(&(n, last)) {
        return x.to_owned();
    }

    let mut cur: Option<Vec<i64>> = None;
    let sn: i64 = (n as f64).sqrt() as i64;
    let mut x = if last == 1 || (sn == last - 1 && sn == 1) {
        0
    } else {
        i64::min(sn, last - 1)
    };
    while x > 0 {
        if let Some(mut m) = dfs(n - x * x, x, memo) {
            m.push(x);
            cur = Some(m);
            break;
        }
        x -= 1;
        // eprintln!("{x} fails for {n}, try {x}-1, last:{last}, {sn}");
    }
    memo.insert((n, last), cur.clone());
    cur
}

fn decompose(n: i64) -> Option<Vec<i64>> {
    // your code
    let mut memo: HashMap<(i64, i64), Option<Vec<i64>>> = HashMap::new();
    for x in (1..n).rev() {
        if let Some(mut res) = dfs(n * n - x * x, x, &mut memo) {
            res.push(x);
            return Some(res);
        }
    }
    None
}

fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1, 3, 5, 8, 49]));
    testing(44, Some(vec![2, 3, 5, 7, 43]));
    testing(625, Some(vec![2, 5, 8, 34, 624]));
    testing(5, Some(vec![3, 4]));
}
