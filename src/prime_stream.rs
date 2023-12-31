fn stream() -> impl Iterator<Item = u32> {
    return PrimeSet {
        primes: vec![2, 3],
        n: 0,
    };
}

struct PrimeSet {
    primes: Vec<u32>,
    n: usize, // generated
}

// Naive and trivial implement
impl Iterator for PrimeSet {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.primes.len() {
            self.n += 1;
            let res = self.primes[self.n - 1];
            // eprintln!("get {res}");
            return Some(res);
        }
        let mut l = self.primes.last().unwrap() + 2;
        loop {
            let bad = self
                .primes
                .iter()
                .take_while(|&&x| x * x <= l)
                .any(|x| l % x == 0);
            // for p in self.primes.iter() {
            //     eprintln!("test {p} for {l}");
            //     if l % p == 0 {
            //         l += 2;
            //         break;
            //     }
            // }
            if !bad {
                // eprintln!("get and push {l}");
                self.primes.push(l);
                self.n += 1;
                break Some(l);
            }
            l += 2;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_segment(start: u32, numbers: [u32; 10]) {
        let mut prime_iterator = stream();
        for _ in 0..start {
            prime_iterator.next();
        }
        for i in numbers {
            assert_eq!(
                Some(i),
                prime_iterator.next(),
                "\nYour result (left) did not match the expected output (right)"
            );
        }
    }

    #[test]
    fn tests() {
        println!("testing segment from 0");
        test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

        println!("testing segment from 10");
        test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);

        println!("testing segment from 100");
        test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);

        println!("testing segment from 1,000");
        test_segment(
            1_000,
            [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017],
        );
    }
}
