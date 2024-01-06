// Duplicate Encoder
fn duplicate_encode(word: &str) -> String {
    use std::collections::HashMap;
    let mut counter = HashMap::<char, usize>::new();
    for c in word.chars() {
        counter
            .entry(c.to_lowercase().next().unwrap())
            .and_modify(|c| *c = *c + 1)
            .or_insert(1);
    }
    word.chars()
        .map(|c| {
            eprintln!("{c:?}");
            let c = counter.get(&c.to_lowercase().next().unwrap()).unwrap();
            match c {
                1 => '(',
                _ => ')',
            }
        })
        .collect()
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}
