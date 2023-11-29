use std::iter::successors;

fn product_fib(prod: u64) -> (u64, u64, bool) {
    successors(Some((0 as u64, 1 as u64)), |&(a, b)| Some((b, a + b)))
        .take_while(|&(a, b)| a * b < prod)
        .last()
        .map(|(a, b)| (b, a + b, b * (a + b) == prod))
        .unwrap()
    // your code: my implementaion
    // let mut a: u64 = 0;
    // let mut b: u64 = 1;
    // loop {
    //     // eprintln!("{a}, {b}");
    //     let c = a * b;
    //     if c == prod {
    //         return (a, b, true);
    //     } else if c < prod {
    //         let tmp = a;
    //         a = b;
    //         b = tmp + b;
    //     } else {
    //         break;
    //     }
    // }
    // (a, b, false)
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
