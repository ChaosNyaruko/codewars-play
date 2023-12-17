fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let a = [1, 2, 3];
    // copied is the same as .map(|&x| x)
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();
    f64::sqrt(f64::powf((p1.0 - p2.0), 2.0) + f64::powf((p1.1 - p2.1), 2.0))
}

fn brutal_closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64), (f64)) {
    let mut res = ((0.0, 0.0), (0.0, 0.0));
    let mut min = f64::MAX;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let dist = distance(p1, p2);
            if dist < min {
                res = (p1, p2);
                min = dist;
            }
        }
    }
    (res.0, res.1, min)
}

fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    // eprintln!("oo, {points:?}");
    let mut x_sorted = Vec::<(f64, f64)>::with_capacity(points.len());
    let mut y_sorted = Vec::<(f64, f64)>::with_capacity(points.len());
    for p in points {
        x_sorted.push(*p);
        y_sorted.push(*p);
    }
    x_sorted.sort_by(|p1, p2| p1.0.partial_cmp(&p2.0).unwrap());
    y_sorted.sort_by(|p1, p2| p1.1.partial_cmp(&p2.1).unwrap());
    // eprintln!("xx, {x_sorted:?}");
    // eprintln!("yy, {y_sorted:?}");
    let (p1, p2, d) = helper(&x_sorted, &y_sorted);
    (p1, p2)
}

fn helper(x_sorted: &[(f64, f64)], y_sorted: &[(f64, f64)]) -> ((f64, f64), (f64, f64), f64) {
    if x_sorted.len() <= 3 {
        return brutal_closest_pair(x_sorted);
    }
    let m = x_sorted.len() / 2;
    let mid = x_sorted[m];
    let x_left = &x_sorted[..m];
    let x_right = &x_sorted[m..];
    let mut y_left = Vec::<(f64, f64)>::with_capacity(m + 1);
    let mut y_right = Vec::<(f64, f64)>::with_capacity(m + 1);
    for p in y_sorted {
        if p.0 <= mid.0 {
            y_left.push(*p);
        } else {
            y_right.push(*p);
        }
    }
    let (pl1, pl2, distl) = helper(x_left, &y_left);
    let (pr1, pr2, distr) = helper(x_right, &y_right);

    let (mut p1, mut p2, mut dist) = if distl < distr {
        (pl1, pl2, distl)
    } else {
        (pr1, pr2, distr)
    };

    let mut band = Vec::<(f64, f64)>::new();
    for p in y_sorted {
        if p.0 > mid.0 - dist && p.0 < mid.0 + dist {
            band.push(*p);
        }
    }
    let band_size = band.len();
    //assert!(band_size <= 6);
    for i in 0..band_size {
        for j in (i + 1)..usize::min(i + 7, band_size) {
            let a = band[i];
            let b = band[j];
            let d = distance(a, b);
            if d < dist {
                // eprintln!("p1:{a},{b}, p2:{x},{y}), dist:{dist}");
                (p1, p2, dist) = (a, b, d)
            }
        }
    }

    // search band
    return (p1, p2, dist);
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::closest_pair;

    type Points = ((f64, f64), (f64, f64));

    fn verify(actual: Points, expected: Points) {
        if actual == expected || (actual.0 == expected.1 && actual.1 == expected.0) {
            assert!(true)
        } else {
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn sample_tests() {
        verify(
            closest_pair(&[(2.0, 2.0), (6.0, 3.0)]),
            ((2.0, 2.0), (6.0, 3.0)),
        );
        verify(
            closest_pair(&[
                (2.0, 2.0),
                (2.0, 8.0),
                (5.0, 5.0),
                (6.0, 3.0),
                (6.0, 7.0),
                (7.0, 4.0),
                (7.0, 9.0),
            ]),
            ((6.0, 3.0), (7.0, 4.0)),
        );
        verify(
            closest_pair(&[
                (2.0, 2.0),
                (2.0, 8.0),
                (5.0, 5.0),
                (5.0, 5.0),
                (6.0, 3.0),
                (6.0, 7.0),
                (7.0, 4.0),
                (7.0, 9.0),
            ]),
            ((5.0, 5.0), (5.0, 5.0)),
        );
    }
}
