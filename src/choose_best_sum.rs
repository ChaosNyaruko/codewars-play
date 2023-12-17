fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    // eprintln!("into helper t:{t}, k:{k}, {ls:?}");
    // your code
    if k > ls.len() as i32 {
        // eprintln!("out helper: not enough ls");
        return -1;
    }
    if k == 0 {
        // eprintln!("out helper: 0 k");
        return 0;
    }
    // if t == 0 {
    //     eprintln!("out helper: 0 t");
    //     return 0;
    // }
    let mut res = -1;

    for (i, x) in ls.iter().enumerate() {
        if t - x >= 0 {
            let tmp = choose_best_sum(t - x, k - 1, &ls[(i + 1)..].to_vec());
            if tmp >= 0 {
                res = i32::max(tmp + x, res);
                // eprintln!("get t:{t}, x:{x}, k:{k}, {ls:?}, {res}");
            }
        }
    }
    res
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

#[test]
fn basics_choose_best_sum() {
    let ts = &vec![73, 81, 87];
    testing(81, 2, ts, -1);

    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(331, 5, ts, -1);
    testing(331, 2, ts, 178);
    testing(331, 4, ts, 331);

    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);

    let ts = &vec![91];
    testing(230, 1, ts, 91);
    testing(230, 2, ts, -1);

    // 331, 2, [91, 74, 73, 85, 73, 81, 87]
    // 331, 4, [91, 74, 73, 85, 73, 81, 87]
    // 331, 5, [91, 74, 73, 85, 73, 81, 87]
}

// assertion failed: `(left == right)`
//  left: `331`,
// right: `-1`
