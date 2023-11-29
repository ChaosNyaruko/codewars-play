mod solution {
    pub fn range_extraction(a: &[i32]) -> String {
        // Your solution here
        let iter = a.iter();
        let mut v = Vec::<i32>::new();
        let mut res = String::from("");
        let mut ranges = Vec::<String>::new();
        for c in iter {
            if v.is_empty() || v.last().unwrap() + 1 == *c {
                v.push(*c);
            } else {
                let mut x = match v.len() {
                    1 => v.first().unwrap().to_string(),
                    2 => {
                        v.first().unwrap().to_string()
                            + &",".to_owned()
                            + &v.last().unwrap().to_string()
                    }
                    _ => {
                        v.first().unwrap().to_string()
                            + &"-".to_owned()
                            + &v.last().unwrap().to_string()
                    }
                };
                ranges.push(x);
                v.clear();
                v.push(*c);
            }
        }
        let mut x = match v.len() {
            1 => v.first().unwrap().to_string(),
            2 => v.first().unwrap().to_string() + &",".to_owned() + &v.last().unwrap().to_string(),
            _ => v.first().unwrap().to_string() + &"-".to_owned() + &v.last().unwrap().to_string(),
        };
        ranges.push(x);
        v.clear();
        // eprintln!("{ranges:?}");
        ranges.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // assert_eq!(solution::range_extraction(&[-6]), "-6");
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
