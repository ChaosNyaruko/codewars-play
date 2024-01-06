use is_prime::*;
use std::collections::HashMap;
use std::mem;
struct MyPrime {
    start: u32,
    end: u32,

    stream: Vec<u32>,
    cursor: usize,
}

impl Iterator for MyPrime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.stream.len() {
            self.extend_to(10000);
        }
        self.cursor += 1;
        Some(self.stream[self.cursor - 1])
    }
}

impl MyPrime {
    fn extend_to(&mut self, more: u32) {
        self.start = self.end;
        self.end = self.start + more;
        let mut sieve = Vec::with_capacity(self.end as usize - self.start as usize);
        sieve.resize(self.end as usize - self.start as usize, false);
        const START: usize = 1;

        let LIMIT = self.end as usize;
        let OFFSET = self.start as usize;
        for xu in START..LIMIT as usize {
            let x = xu as u32;
            if x * x >= LIMIT as u32 {
                break;
            }
            for yu in START..LIMIT {
                let y: u32 = yu as u32;
                if y * y > LIMIT as u32 {
                    break;
                }

                // Condition 1
                let n: u32 = (4 * x * x) + (y * y);
                if n >= self.start && n < LIMIT as u32 && (n % 12 == 1 || n % 12 == 5) {
                    sieve[n as usize - OFFSET] ^= true;
                }
                // if n == 25 {
                //     eprintln!(
                //         "c1 {x} {y}, sieve[{}]={}",
                //         n as usize - OFFSET,
                //         sieve[n as usize - OFFSET]
                //     );
                // }

                // Condition 2
                let n: u32 = (3 * x * x) + (y * y);
                if n >= self.start as u32 && n < LIMIT as u32 && n % 12 == 7 {
                    sieve[n as usize - OFFSET] ^= true;
                }
                // if n == 25 {
                //     eprintln!(
                //         "c2 {x} {y}, sieve[{}]={}",
                //         n as usize - OFFSET,
                //         sieve[n as usize - OFFSET]
                //     );
                // }

                // Condition 3
                if 3 * x * x > y * y {
                    let n: u32 = (3 * x * x) - (y * y);
                    if x > y && n >= self.start && n < LIMIT as u32 && n % 12 == 11 {
                        sieve[n as usize - OFFSET] ^= true
                    }
                    // if n == 25 {
                    //     eprintln!(
                    //         "c3 {x} {y}, sieve[{}]={}",
                    //         n as usize - OFFSET,
                    //         sieve[n as usize - OFFSET]
                    //     );
                    // }
                }
            }
        }

        // Mark all multiples of squares as non-prime
        let mut r_cursor = 2;
        let l = self.stream.len();
        while r_cursor < l {
            let r: u32 = self.stream[r_cursor];
            if r * r >= LIMIT as u32 {
                break;
            }
            let mut i: u32 = r * r;
            loop {
                if i >= LIMIT as u32 {
                    break;
                }

                if i >= OFFSET as u32 {
                    sieve[i as usize - OFFSET] = false;
                }

                i += r * r;
            }
            r_cursor += 1;
        }
        let mut r: u64 = self.start.into();
        loop {
            if r * r >= LIMIT as u64 {
                break;
            }

            if sieve[r as usize - OFFSET] {
                let mut i: u64 = r * r;
                loop {
                    if i >= LIMIT as u64 {
                        break;
                    }

                    sieve[i as usize] = false;

                    i += r * r;
                }
            }

            r += 1;
        }

        let mut extended = false;
        for a in self.start..self.end {
            if sieve[a as usize - OFFSET] {
                self.stream.push(a);
                extended = true;
            }
        }
        if !extended {
            return self.extend_to(more);
        }
    }
}

fn sieve_of_atkin() {
    const LIMIT: usize = 1000;
    let mut sieve: [bool; LIMIT + 1] = [false; LIMIT + 1];

    // 2 and 3 are known to be prime
    if LIMIT > 2 {
        sieve[2] = true;
    }
    if LIMIT > 3 {
        sieve[3] = true;
    }

    /*
    Mark sieve[n] true if one of these condtiions are met

    a) n = (4 * x * x) + (y * y) has an odd number of solutions
        and n % 12 == 1 or n % 12 == 5
    b) n = (3 * x * x) + (y * y) has an odd number of solutions
        and n % 12 == 7
    c) n = (3 * x * x) - (y * y) has an odd number of solutions,
        x > y and n % 12 == 11

        @See https://www.geeksforgeeks.org/sieve-of-atkin/ for examples
    */
    const START: usize = 1;
    for xu in START..LIMIT {
        let x: i32 = xu as i32;
        if x * x > LIMIT as i32 {
            break;
        }
        for yu in START..LIMIT {
            let y: i32 = yu as i32;
            if y * y > LIMIT as i32 {
                break;
            }

            // Condition 1
            let n: i32 = (4 * x * x) + (y * y);
            if n <= LIMIT as i32 && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize] ^= true;
            }
            if n == 25 {
                eprintln!("c1 {x} {y}, sieve[{}]={}", n, sieve[n as usize]);
            }

            // Condition 2
            let n: i32 = (3 * x * x) + (y * y);
            if n <= LIMIT as i32 && n % 12 == 7 {
                sieve[n as usize] ^= true;
            }
            if n == 25 {
                eprintln!("c2 {x} {y}, sieve[{}]={}", n, sieve[n as usize]);
            }

            // Condition 3
            let n: i32 = (3 * x * x) - (y * y);
            if x > y && n <= LIMIT as i32 && n % 12 == 11 {
                sieve[n as usize] ^= true
            }
            if n == 25 {
                eprintln!("c3 {x} {y}, sieve[{}]={}", n, sieve[n as usize]);
            }
        }
    }

    // Mark all multiples of squares as non-prime
    let mut r: i32 = 5;
    loop {
        if r * r > LIMIT as i32 {
            break;
        }

        if sieve[r as usize] {
            let mut i: i32 = r * r;
            loop {
                if i > LIMIT as i32 {
                    break;
                }

                sieve[i as usize] = false;

                i += r * r;
            }
        }

        r += 1;
    }

    // Print primes contained within sieve's index.
    let mut i = 0;
    for a in START..=LIMIT {
        if sieve[a] {
            i += 1;
            println!("{i}:{}", a)
        }
    }
}

#[derive(Debug, Clone)]
struct Primes {
    i: usize,
    curr_candidate: u32,
    next_relevant_prime: u32,
    next_relevant_prime_squared: u32,
    sieve: HashMap<u32, u32>,
    initial_primes: Vec<u32>,
    internal_primes: Box<Option<Primes>>,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            i: 0,
            curr_candidate: 7,
            next_relevant_prime: 0,
            next_relevant_prime_squared: 0,
            sieve: HashMap::new(),
            initial_primes: vec![2, 3, 5, 7],
            internal_primes: Box::new(None),
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.initial_primes.len();
        let mut internal_primes;
        if self.i < len {
            self.i += 1;
            return Some(self.initial_primes[self.i - 1]);
        } else if self.i == len {
            self.i += 1;
            internal_primes = Primes::new();
            internal_primes.next(); // skip 2
            self.next_relevant_prime = internal_primes.next().unwrap();
            self.next_relevant_prime_squared = self.next_relevant_prime.pow(2);
        } else {
            internal_primes = mem::replace(&mut self.internal_primes, Box::new(None)).unwrap();
        }
        let mut i = self.curr_candidate;
        loop {
            i += 2;
            let step;
            if self.sieve.contains_key(&i) {
                // composite
                step = self.sieve.remove(&i).unwrap();
            } else if i < self.next_relevant_prime_squared {
                // prime
                // save state for next round
                self.curr_candidate = i;
                self.internal_primes = Box::new(Some(internal_primes));
                return Some(i);
            } else {
                // i == next_relevant_prime_squared
                step = 2 * self.next_relevant_prime;
                self.next_relevant_prime = internal_primes.next().unwrap();
                self.next_relevant_prime_squared = self.next_relevant_prime.pow(2);
            }
            let mut j = i;
            j += step;
            while self.sieve.contains_key(&j) {
                j += step;
            }
            self.sieve.insert(j, step);
        }
    }
}

fn stream() -> impl Iterator<Item = u32> {
    // Primes::new()
    // primal::Primes::all()
    MyPrime {
        start: 1,
        end: 24,
        stream: vec![2, 3, 5, 7, 11, 13, 17, 19, 23],
        cursor: 0,
    }
}
// fn main() {
//     let mut primes = Primes::new();
//     for _i in 0..99_999 {
//         primes.next();
//     }
//     println!("The 100,000th prime is {}", primes.next().unwrap())
// }
#[cfg(test)]
mod tests {
    use crate::prime_sieve_stream::is_prime;

    use super::*;

    #[test]
    fn raw_atkin() {
        sieve_of_atkin();
        let mut s = stream();
        for i in 0..200 {
            let x = s.next().unwrap();
            if is_prime(x.into()) {
                eprintln!("mine: {i}: {}", x)
            } else {
                unreachable!("[NOT]mine: {i}: {}", x)
            }
        }
    }

    fn test_segment(start: usize, numbers: [u32; 10]) {
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
