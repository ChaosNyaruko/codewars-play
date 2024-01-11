fn xo(string: &'static str) -> bool {
    let mut x = 0;
    let mut y = 0;
    for i in string.to_lowercase().chars() {
        if i == 'x' {
            x += 1;
        }
        if i == 'o' {
            y += 1;
        }
    }
    let s2 = "ababa".to_lowercase();
    println!("{}", s2.matches("aba").count());
    let s2 = "abaaba".to_lowercase();
    println!("{}", s2.matches("aba").count());
    let a = string.chars().fold(0, |delta, c| match c {
        'x' | 'X' => delta + 1,
        'o' | 'O' => delta - 1,
        _ => delta,
    });
    return a == 0;
    x == y
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
