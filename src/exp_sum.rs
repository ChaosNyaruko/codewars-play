fn exp_sum(n: u64) -> u64 {
    use std::collections::HashMap;
    let mut memo: HashMap<(i64, i64), u64> = HashMap::new();
    fn p(k: i64, n: i64, m: &mut HashMap<(i64, i64), u64>) -> u64 {
        if k == 0 && n == 0 {
            return 1;
        }
        if k <= 0 || n <= 0 {
            return 0;
        }

        if let Some(res) = m.get(&(k, n)) {
            return *res;
        }
        let res = p(k, n - k, m) + p(k - 1, n - 1, m);
        m.insert((k, n), res);
        return res;
    }
    (0..=n).map(|k| p(k as i64, n as i64, &mut memo)).sum()
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
        assert_eq!(exp_sum(50), 204226);
        assert_eq!(exp_sum(80), 15796476);
        assert_eq!(exp_sum(100), 190569292);
    }
}
