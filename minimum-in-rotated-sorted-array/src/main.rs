use itertools::Itertools;
use superslice::*;

fn rotation_index(vec: &Vec<i32>) -> i32 {
    let reversed_vec =
        vec
        .iter()
        .rev();
    let cooked =
        reversed_vec
        .zip(vec.iter())
        .take(vec.len() / 2)
        .collect::<Vec<_>>();
    println!("{:?}", cooked);
    let compare_tuples = | e : &(&i32, &i32) | {
        if e.0 < e.1 {
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

    result as i32
}

fn main() {
    assert_eq!(rotation_index(&vec![1, 2, 3, 4]), 0);
    // A
    assert_eq!(rotation_index(&vec![2, 3, 4, 1]), 1);
    assert_eq!(rotation_index(&vec![3, 4, 1, 2]), 2);
    // B
    assert_eq!(rotation_index(&vec![4, 1, 2, 3]), 1);
    // A ~ B -> this is represented by a number 1
    // there will be N / 2 numbers to denominate rotation with the possibility
    // of losing the direction information, which can be checked anyway

    assert_eq!(rotation_index(&vec![1, 2, 3, 4, 5]), 0);
}
