
fn count_duplicates(text: &str) -> u32 {
    use std::collections::HashMap;
    // Your code goes here
    let mut record: HashMap<char, u32> = HashMap::new();
    let text = text.to_lowercase();
    for c in text.chars() {
        record.entry(c).and_modify(|c| *c = *c + 1).or_insert(1);
    }
    (record.values().filter(|&&x| x > 1).count()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
