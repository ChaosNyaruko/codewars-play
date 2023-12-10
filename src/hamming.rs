fn hamming(n: usize) -> u64 {
    let (mut p2, mut p3, mut p5) = (0, 0, 0);
    let mut dp = vec![1u64];
    // x Vec<u64>
    // for i in x i -> &u64
    for _ in 1..n {
        // find min(dp[p2]*2, dp[p3]*3, dp[p5]*5)
        let min = [dp[p2] * 2, dp[p3] * 3, dp[p5] * 5]
            .into_iter()
            .min()
            .unwrap();
        if dp[p2] * 2 == min {
            p2 += 1;
        }
        if dp[p3] * 3 == min {
            p3 += 1;
        }
        if dp[p5] * 5 == min {
            p5 += 1;
        }
        dp.push(min);
    }
    *dp.last().unwrap()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn sample_tests() {
        assert_eq!(hamming(1), 1);
        assert_eq!(hamming(2), 2);
        assert_eq!(hamming(3), 3);
        assert_eq!(hamming(4), 4);
        assert_eq!(hamming(5), 5);
        assert_eq!(hamming(6), 6);
        assert_eq!(hamming(7), 8);
        assert_eq!(hamming(8), 9);
        assert_eq!(hamming(9), 10);
        assert_eq!(hamming(10), 12);
        assert_eq!(hamming(11), 15);
        assert_eq!(hamming(12), 16);
        assert_eq!(hamming(13), 18);
        assert_eq!(hamming(14), 20);
        assert_eq!(hamming(15), 24);
        assert_eq!(hamming(16), 25);
        assert_eq!(hamming(17), 27);
        assert_eq!(hamming(18), 30);
        assert_eq!(hamming(19), 32);
    }
}
