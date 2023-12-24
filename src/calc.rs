/// Evaluate mathematical expression
/// Instructions
///
/// Given a mathematical expression as a string you must return the result as a number.
/// Numbers
///
/// Number may be both whole numbers and/or decimal numbers. The same goes for the returned result.
/// Operators
///
/// You need to support the following mathematical operators:
///
///     Multiplication *
///     Division / (as floating point division)
///     Addition +
///     Subtraction -
///
/// Operators are always evaluated from left-to-right, and * and / must be evaluated before + and -.
/// Parentheses
///
/// You need to support multiple levels of nested parentheses, ex. (2 / (2 + 3.33) * 4) - -6
/// Whitespace
///
/// There may or may not be whitespace between numbers and operators.
///
/// An addition to this rule is that the minus sign (-) used for negating numbers and parentheses will never be separated by whitespace. I.e all of the following are valid expressions.
///
/// 1-1    // 0
/// 1 -1   // 0
/// 1- 1   // 0
/// 1 - 1  // 0
/// 1- -1  // 2
/// 1 - -1 // 2
/// 1--1   // 2
///
/// 6 + -(4)   // 2
/// 6 + -( -4) // 10
///
/// And the following are invalid expressions
///
/// 1 - - 1    // Invalid
/// 1- - 1     // Invalid
/// 6 + - (4)  // Invalid
/// 6 + -(- 4) // Invalid
///
/// Validation
///
/// You do not need to worry about validation - you will only receive valid mathematical expressions following the above rules.
use std::collections::VecDeque;
fn calc(expr: &str) -> f64 {
    let mut pos = 0;
    println!("expr: {}, len:{}", expr, expr.len());
    let res = helper(expr, &mut pos);
    assert_eq!(pos, expr.len());
    return res;
}

fn read_spaces(expr: &str, pos: &mut usize) {
    while *pos < expr.len() {
        if !cur(expr, pos).is_whitespace() && cur(expr, pos) != '\n' {
            break;
        };
        *pos += 1;
    }
}

fn cur(expr: &str, pos: &usize) -> char {
    if *pos >= expr.len() {
        return ' ';
    }
    expr.chars()
        .nth(*pos)
        .expect(&format!("pos in cur {expr}, len:{}, {pos}", expr.len()))
}

fn parse_unary(expr: &str, sign: char, pos: &mut usize) -> f64 {
    let sign = match sign {
        '+' => 1.0,
        '-' => -1.0,
        _ => unreachable!(),
    };
    let n = cur(expr, pos);
    let mut neg = 0.0;
    if n.is_digit(10) {
        neg = parse_num(expr, pos);
    } else if n == '(' {
        neg = helper(expr, pos);
    } else {
        unreachable!()
    }
    neg * sign
}

fn parse_num(expr: &str, pos: &mut usize) -> f64 {
    read_spaces(expr, pos);
    let start: usize = *pos;
    while *pos < expr.len() {
        let c = cur(expr, pos);
        if c == '.' || (c >= '0' && c <= '9') {
            *pos += 1;
        } else {
            break;
        }
    }

    let res = expr[start..*pos].parse().expect("parse f64");
    read_spaces(expr, pos);
    // eprintln!("start parsing a num: {start}..{pos}");
    return res;
}

fn helper(expr: &str, pos: &mut usize) -> f64 {
    let start = *pos;
    // eprintln!("a helper start! from {start}");
    let test = &expr[*pos..];
    let mut operands = VecDeque::<f64>::new();
    let mut ops = VecDeque::<char>::new();
    let mut last = None;
    while *pos < expr.len() {
        read_spaces(expr, pos);
        let c = cur(expr, pos);
        // 1 + 1
        // eprintln!("read a {c}");
        if c == '(' {
            // the calling helper is responsible for the closing parenthesis
            *pos += 1;
            let a = helper(expr, pos);
            // eprintln!("paring (), got:{a:?}");
            operands.push_back(a);
            last = Some(')');
        } else if c <= '9' && c >= '0' {
            let a = parse_num(expr, pos);
            operands.push_back(a);
            last = Some('1');
            // eprintln!("parse a num: {a:?}, from {pos}:{c}");
        } else if c == '+' || c == '-' {
            // eprintln!("parsing a +");
            *pos += 1;
            match last {
                Some(x) => {
                    match x {
                        ')' | '1' => {
                            ops.push_back(c);
                            last = Some('+');
                        } // +/- is binary
                        '+' => {
                            operands.push_back(parse_unary(expr, c, pos));
                            last = Some('1');
                        } // unary
                        _ => unreachable!("{x}"),
                    }
                }
                None => {
                    operands.push_back(parse_unary(expr, c, pos)); // unary
                    last = Some('1');
                }
            };
        } else if c == '*' || c == '/' {
            // assert_expr_eq!("2 /(2+3)*4", 1.6);
            *pos += 1;
            let x = operands.pop_back().expect("pop back operand in * and /");
            read_spaces(expr, pos);
            // eprintln!("processing {c} x:{x}, pos:{pos}");
            let next = cur(expr, pos);
            let y = match next {
                '0'..='9' => parse_num(expr, pos),
                '(' => {
                    *pos += 1;
                    helper(expr, pos)
                }
                '-' | '+' => {
                    *pos += 1;
                    parse_unary(expr, next, pos)
                }
                _ => helper(expr, pos),
            };
            let z = match c {
                '*' => x * y,
                '/' => x / y,
                _ => unreachable!(),
            };
            // eprintln!("processing {c} x:{x}, {y}, {z}, pos:{pos}");
            operands.push_back(z);
        } else {
            // assert_eq!(c, ')');
            *pos += 1;
            last = Some(c);
            if c == ')' {
                // eprintln!("break )! operands: {operands:?}, ops:{ops:?}");
                break;
            } else if !c.is_whitespace() {
                unreachable!("get a >????xx{c}xx");
            }
        }
        // eprintln!("operands: {operands:?}, ops:{ops:?}");
    }
    // assert_eq!(operands.len(), 1 + ops.len());
    // eprintln!("{test}:, operands: {operands:?}, ops:{ops:?}");
    while !ops.is_empty() {
        let op = ops.pop_front().expect("pop op");
        let o1 = operands.pop_front().expect("pop o1"); // at least one
        let o2 = operands.pop_front().expect("pop o2");
        let x = match op {
            '+' => o1 + o2,
            '-' => o1 - o2,
            '*' => panic!("* / should be calculated during the iteration"), //o1 * o2,
            '/' => panic!("* / should be calculated during the iteration"), //o1 / o2
            _ => panic!("unexpected operation"),
        };
        // eprintln!("{o1:?}{op:?}{o2:?}={x}");
        operands.push_front(x);
    }
    // assert_eq!(operands.len(), 1);
    // assert_eq!(ops.len(), 0);
    let res = operands.pop_front().expect("pop res");
    // eprintln!("a helper done! {res} from {start}");
    res
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
        assert_expr_eq!("3.5", 3.5);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
        assert_expr_eq!("(12 * 123)", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
        assert_expr_eq!("-5 + 2", -3.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3", 4.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 /(2+3)*4", 1.6);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }

    #[test]
    fn random() {
        assert_expr_eq!(
            "((675 - 405 - 135) - (243 - -486 - 648) - (13.5 * 216 / 108))",
            27.0
        );
        assert_expr_eq!("(3 - (12 / -1.5 * 0.75) - (24 - -48 - 64))", 1.0);
    }

    #[test]
    fn extend() {
        // assert_expr_eq!(
        //     "123.45*(678.90 / (-2.5+ 11.5)-(80 -19) *33.25) / 20 + 11",
        //     -12042.760875
        // );
        // "(123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) - (123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) + (13 - 2)/ -(-11)"
        assert_expr_eq!("(123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) - (123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) + (13 - 2)/ -(-11)", 1.0);
        // assert_expr_eq!("(123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) - (123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) + (13 - 2)/ -(-11)", 1.0);
        // (123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) - (123.45*(678.90 / (-2.5+ 11.5)-(((80 -(19))) *33.25)) / 20) + (13 - 2)/ -(-11)
    }
}
