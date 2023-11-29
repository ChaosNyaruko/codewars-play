fn zeros(n: u64) -> u64 {
    // your code here
    // how many 5s does n have?
    // return std::iter::successors(Some(n/5), |&n| Some(n/5)).take_while(|&n| n > 0).sum();
    let mut res = 0;
    let mut n = n;
    while n > 0 {
        res += n / 5;
        n /= 5;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }    
}
