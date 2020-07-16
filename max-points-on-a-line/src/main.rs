use std::collections as ds;
use std::convert::TryInto;

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {

    let mut ans: usize = usize::min_value();

    for from in &points {

        let mut slope_cache: ds::HashMap<(i32, i32), usize> =
            ds::HashMap::new();

        let mut same: usize = 0;

        for to in &points {

            if from == to {
                same += 1;
                println!("{}", same);
                continue;
            }

            let reduced_slope = reduce(from, to);
            slope_cache.insert(reduced_slope,
                               *slope_cache.get(&reduced_slope)
                               .unwrap_or(&0) + 1);
        }

        println!("A");

        println!("{}", same);

        for e in slope_cache {
            println!("{:?}", e);

            ans = std::cmp::max(ans, e.1 + same);
        }

        ans = std::cmp::max(ans, same);

        println!("B");
    }

    ans as i32
}

fn reduce(from: &Vec<i32>, to: &Vec<i32>) -> (i32, i32) {
    let run = to[0] - from[0];
    let rise = to[1] - from[1];
    // This code is taken from num-iter crate
    let mut m = rise;
    let mut n = run;
    if m == 0 || n == 0 {
        let gcd = (m | n).abs();
        return (rise / gcd, run / gcd);
    }
    let shift = (m | n).trailing_zeros();
    if m == i32::min_value() || n == i32::min_value() {
        let tempo: i32 = (1 << shift);
        let gcd = tempo.abs();
        return (rise / gcd, run / gcd);
    }
    m = m.abs();
    n = n.abs();
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();
    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    let gcd = m << shift;

    (rise / gcd, run / gcd)
}

fn main() {
    let start = vec![0, 0];
    for end in vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1],
                    vec![2, 2], vec![2, -2], vec![-2, 2], vec![-2, -2]]
    {
        let reduced = reduce(&start, &end);
        assert_eq!(reduced.0 < 2 && reduced.1 < 2, true);
    }

    // this should panic
    // let reduced = reduce(&vec![1,1], &vec![1,1]);
    // assert_eq!(reduced.0 < 2 && reduced.1 < 2, true);

    let value: Vec<_> = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(3, max_points(value));

    let value: Vec<_> = vec![vec![1,1], vec![2,2]];
    assert_eq!(2, max_points(value));

    let value: Vec<_> = vec![vec![0, 0], vec![0, 0]];
    assert_eq!(2, max_points(value));


}
