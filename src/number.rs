/// Number of People in the Bus
fn number(bus_stops: &[(i32, i32)]) -> i32 {
    // let mut res = 0;
    // for (on, off) in bus_stops {
    //     res += on - off;
    // }
    bus_stops.iter().fold(0, |cur, (on, off)| cur + on - off);
    bus_stops.iter().fold(0, |cur, &(on, off)| cur + on - off);
    bus_stops.iter().fold(0, |cur, &x| cur + x.0 - x.1);
    bus_stops.iter().fold(0, |cur, x| cur + x.0 - x.1)
}

#[test]
fn returns_expected() {
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
        17
    );
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
        21
    );
}
