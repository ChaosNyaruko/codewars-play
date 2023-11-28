fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    // The following code just does the _dedup_ things.
    // let mut v: Vec<_> = sequence.into_iter().collect();
    // v.dedup();
    // return v;
    // 
    let mut res: Vec<T::Item> = Vec::new();
    let mut iter = sequence.into_iter();
    while let Some(a) = iter.next() {
        eprintln!("{a:?}");
        if let Some(b) = res.last() {
            if *b == a {
                continue;
            }
        }
        res.push(a);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }
}
