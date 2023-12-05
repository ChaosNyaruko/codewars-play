// Consider a "word" as any sequence of capital letters A-Z (not limited to just "dictionary words"). For any word with at least two different letters, there are other words composed of the same letters but in a different order (for instance, STATIONARILY/ANTIROYALIST, which happen to both be dictionary words; for our purposes "AAIILNORSTTY" is also a "word" composed of the same letters as these two).

// We can then assign a number to every word, based on where it falls in an alphabetically sorted list of all words made up of the same group of letters. One way to do this would be to generate the entire list of words and find the desired one, but this would be slow if the word is long.

// Given a word, return its number. Your function should be able to accept any word 25 letters or less in length (possibly with some letters repeated), and take no more than 500 milliseconds to run. To compare, when the solution code runs the 27 test cases in JS, it takes 101ms.

// For very large words, you'll run into number precision issues in JS (if the word's position is greater than 2^53). For the JS tests with large positions, there's some leeway (.000000001%). If you feel like you're getting it right for the smaller ranks, and only failing by rounding on the larger, submit a couple more times and see if it takes.

// Python, Java and Haskell have arbitrary integer precision, so you must be precise in those languages (unless someone corrects me).

// C# is using a long, which may not have the best precision, but the tests are locked so we can't change it.

// Sample words, with their rank:
// ABAB = 2
// AAAB = 1
// BAAA = 4
// QUESTION = 24572
// BOOKKEEPER = 10743


fn other_list_position(word: &str) -> u128 {
    use std::collections::HashMap;
    let mut pos = 1;
    let mut perm = 1;
    let mut cnt = HashMap::<u8, u128>::new();
    
    for (i, c1) in word.bytes().rev().enumerate() {
        *cnt.entry(c1).or_default() += 1;
        for c2 in cnt.keys() {
            if *c2 < c1 {
                pos += perm * cnt[&c2] / cnt[&c1];
            }
        }
        perm = perm * (i as u128 + 1) / cnt[&c1];
    }
    
    pos
}

fn factorial(n: u128) -> u128 {
    let mut res = 1;
    for x in 1..=n {
        res *= x;
    }
    res
}

fn list_position(word: &str) -> u128 {
    use std::collections::BTreeMap;
    if word.len() == 1 {
        return 1;
    }
    let mut counter: BTreeMap<u8, usize> = BTreeMap::new();
    for c in word.as_bytes() {
        counter.entry(*c).and_modify(|c| *c = *c + 1).or_insert(1);
    }
    // Calc the number of _smaller_ permutations,
    // Example: word DACB
    // count(Axxx) + count(Cxxx) + count(Bxxx) is what we want.
    let mut n = word.len() - 1;
    let top = factorial(n.try_into().unwrap());


    let x = word.bytes().nth(0).unwrap(); // word.len() is guaranteed to be at least 1.
    let mut count = 0;
    for k in counter.keys() {
        if *k < x {
            let mut bottom = 1;
            for (y, c) in &counter {
                let mut c = (*c).try_into().unwrap();
                if y == k {
                    if c == 1 {
                        c = 1;
                    } else {
                        c -= 1;
                    }
                }
                bottom *= factorial(c);
            }
            count += top / bottom;
        }
    }
    count += list_position(&word[1..]);
    count
}

fn naive_list_position(word: &str) -> u128 {
    let word =  String::from(word);
    let origin = word.as_bytes();
    let mut word = Vec::<u8>::with_capacity(origin.len());
    for c in origin {
        word.push(*c);
    }
    word.sort();
    // eprintln!("{word:?}");

    let mut count = 1;
    fn next_permutation(x: &mut[u8]) -> bool {
        let mut i = x.len() - 1;
        while i > 0 && x[i] <= x[i-1] {
            i -= 1;
        }
        if i == 0 {
            // x is the last permutation.
            return true;
        }
        let pivot = i - 1;
        i = x.len() - 1;
        while x[i] <= x[pivot] {
            i -= 1;
        }
        // swap x[i] with x[swap]
        x.swap(i, pivot);

        // reverse the suffix
        x[pivot+1..].reverse();
        return false;
    }

    while word != origin {
        // eprintln!("{word:?}");
        let stop = next_permutation(&mut word);
        count += 1;
        if stop {
            break;
        }
    }

    count
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::list_position;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    #[test]
    fn sample_tests() {
        let test_data = [
            (                  "A", 1),
            (               "ABAB", 2),
            (               "AAAB", 1),
            (               "BAAA", 4),
            (               "YMYM", 5),
            (           "QUESTION", 24572),
            (         "BOOKKEEPER", 10743),
      ("IMMUNOELECTROPHORETICALLY", 718393983731145698173),
        ];
        for (word, expected) in test_data {
            assert_eq!(list_position(word), 
                       expected,
                       "\nYour result (left) did not match the expected output (right) for the input: \"{word}\"");
        }
        
    }
}
