use std::iter::Peekable;

fn calc(expr: &str) -> f64 {
    let s = expr.replace(' ', "");
    eval(&mut s.chars().peekable())
}

fn eval<I>(chars: &mut Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    let mut e = term(chars);
    loop {
        if matches!(chars.peek(), None | Some(')')) {
            break e;
        }
        let op = chars.next().unwrap();
        match op {
            '+' => e += term(chars),
            '-' => e -= term(chars),
            _ => unreachable!(),
        }
    }
}

fn term<I>(chars: &mut Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    let mut f = factor(chars);
    loop {
        if matches!(chars.peek(), None | Some(')' | '+' | '-')) {
            break f;
        }
        let op = chars.next().unwrap();
        match op {
            '*' => f *= factor(chars),
            '/' => f /= factor(chars),
            _ => unreachable!(),
        }
    }
}

fn factor<I>(chars: &mut Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    match chars.peek().unwrap() {
        '-' => {
            chars.next();
            -factor(chars)
        }
        c if c.is_digit(10) => literal(chars),
        '(' => {
            chars.next();
            let v = eval(chars);
            chars.next();
            v
        }
        _ => unreachable!(),
    }
}

fn literal<I>(chars: &mut Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    while let Some(&c) = chars.peek().filter(|&&c| c == '.' || c.is_digit(10)) {
        s.push(c);
        chars.next();
    }
    s.parse().unwrap()
}
