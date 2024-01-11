/// Growth of a Population
fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
  // your code
    let mut p0 = p0 as f64;
    let percent = percent / 100.0;
    let mut y = 0;
    let p:f64 = p.into();
    while p0 < p {
        p0 = p0 * (1.0 + percent) + aug as f64;
        y+=1;
    }
    y
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans =nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
        
    }
}
