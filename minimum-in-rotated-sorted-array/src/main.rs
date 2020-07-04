use itertools::Itertools;
use superslice::*;

fn solve(vec: &Vec<i32>) {
    let reversed_vec =
        vec
        .iter()
        .rev();

    let cooked =
        reversed_vec
        .zip(vec.iter())
        .take(vec.len() / 2)
        .collect::<Vec<_>>()
        ;

    println!("{:?}", cooked);

    let compare_tuples = | e : &(&i32, &i32) | {
        if e.0 > e.1 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    };

    let result =
        cooked
        .lower_bound_by(
            compare_tuples
        );

    println!("{:?}", result);
}

fn main() {
    solve(&vec![1, 2, 3]);
}
