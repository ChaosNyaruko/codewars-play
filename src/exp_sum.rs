use std::collections::HashMap;
fn p(k: i64, n: i64, memo: &mut HashMap<(i64, i64), u64>) -> u64 {
    if k == 0 && n == 0 {
        return 1;
    }
    if k <= 0 || n <= 0 {
        return 0;
    }
    if let Some(x) = memo.get(&(k, n)) {
        return *x;
    }

    let res = p(k, n - k, memo) + p(k - 1, n - 1, memo);
    memo.insert((k, n), res);
    res
}

fn _p(k: u64, n: u64, start: u64, memo: &mut HashMap<(u64, u64, u64), u64>) -> u64 {
    if k == 0 && n == 0 {
        return 1;
    }

    if n < start {
        return 0;
    }

    if k == 0 && n != 0 {
        return 0;
    }

    if let Some(res) = memo.get(&(k, n, start)) {
        return *res;
    }
    let mut res = 0u64;
    for i in start..=n {
        if n >= i {
            let a = k - 1;
            let b = n - i;
            let tmp = _p(a, b, i, memo);
            // eprintln!("p({a},{b},{i},{max}) = {tmp}");
            res += tmp;
        }
    }
    memo.insert((k, n, start), res);
    return res;
}

fn exp_sum(n: u64) -> u64 {
    let mut res: u64 = 0;
    let mut memo: HashMap<(i64, i64), u64> = HashMap::new();
    for k in 1..=n {
        let tmp = p(k as i64, n as i64, &mut memo);
        res += tmp;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sample_tests() {
        assert_eq!(exp_sum(1), 1);
        assert_eq!(exp_sum(2), 2);
        assert_eq!(exp_sum(3), 3);
        assert_eq!(exp_sum(4), 5);
        assert_eq!(exp_sum(5), 7);
        assert_eq!(exp_sum(10), 42);
    }

    #[test]
    fn p_test() {
        // assert_eq!(p(3, 4, 1, 4), 1);
        // assert_eq!(p(1, 2, 1, 2), 1);
    }
}
