use superslice::*;
use std::cmp;

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
            cmp::Ordering::Less
        } else if e.0 == e.1 {
            cmp::Ordering::Equal
        } else {
            cmp::Ordering::Greater
        }
    };
    let result =
        cooked
        .upper_bound_by(
            compare_tuples
        );
    result as i32
}

fn minimum_in_rotated_array(rotation_point: i32, vec: &Vec<i32>) -> i32 {
    if rotation_point == 0 {
        vec[0]
    } else {
        std::cmp::min(vec[rotation_point as usize],
                  vec[vec.len() - rotation_point as usize])
    }
}

mod manual;

fn main() {
    // not moved at all
    assert_eq!(rotation_index(&vec![1, 2, 3, 4]), 0);
    assert_eq!(minimum_in_rotated_array(0, &vec![1, 2, 3, 4]), 1);
    // moved around the first hole
    assert_eq!(rotation_index(&vec![2, 3, 4, 1]), 1);
    assert_eq!(minimum_in_rotated_array(1, &vec![2, 3, 4, 1]), 1);
    // moved around the central hole or element, as the index is out of range
    // [0..N / 2 - 1]
    assert_eq!(rotation_index(&vec![3, 4, 1, 2]), 2);
    assert_eq!(minimum_in_rotated_array(2, &vec![3, 4, 1, 2]), 1);
    // moved around the third hole, which is symmetric with the first hole,
    // the function chooses the lower number, hence 1
    assert_eq!(rotation_index(&vec![4, 1, 2, 3]), 1);
    assert_eq!(minimum_in_rotated_array(1, &vec![4, 1, 2, 3]), 1);

    // not moved at all
    assert_eq!(rotation_index(&vec![1, 2, 3, 4, 5]), 0);
    assert_eq!(minimum_in_rotated_array(0, &vec![1, 2, 3, 4, 5]), 1);
    // moved aroung the first hole
    assert_eq!(rotation_index(&vec![2, 3, 4, 5, 1]), 1);
    assert_eq!(minimum_in_rotated_array(1, &vec![2, 3, 4, 5, 1]), 1);
    // moved around the hole that is located to the left from the central
    // element (index out of range)
    assert_eq!(rotation_index(&vec![3, 4, 5, 1, 2]), 2);
    assert_eq!(minimum_in_rotated_array(2, &vec![3, 4, 5, 1, 2]), 1);
    // moved around the hole that is located to the right from the central
    // element (index out of range)
    assert_eq!(rotation_index(&vec![4, 5, 1, 2, 3]), 2);
    assert_eq!(minimum_in_rotated_array(2, &vec![4, 5, 1, 2, 3]), 1);
    // symmetric to the prev 1-case: if reversed around the first hole and is of
    // form XY, then current vec is YX, that is symmetric rotation.
    assert_eq!(rotation_index(&vec![5, 1, 2, 3, 4]), 1);
    assert_eq!(minimum_in_rotated_array(1, &vec![5, 1, 2, 3, 4]), 1);
    println!("Work in progress tests");
    assert_eq!(rotation_index(&vec![4, 5, 6, 4, 4]), 2);
    assert_eq!(minimum_in_rotated_array(2, &vec![4, 5, 6, 4, 4]), 4);

    /// ===================================================================

    // HOUSTON WE HAVE A PROBLEM;
    // there is really no relation of order in the cooked vector;
    // here is why;
    // a..X..b..b..Y..a
    // (a, a) shows no sign that the sequencde is not sorted, so we might skip
    // it and go rigth - or left; but we need to do both, which makes this approach
    // not work for O(log N).
    // (b, b) - the same, so go further - to the right
    // we missed (X, Y), which was not sorted!
    // in this algorithm, we are trying to find the maximum index of a pair that breaks
    // the order in the array.

    // here is what the algorithm tries to do; fold the vector in half and search
    // it for the maximum index that corresponds to a pair that is out of order.
    // the trick is that we reduce the time to search such a structure in half
    // compared to naively looking for every element and trying to update the max
    // value; the worst case will be when the rightmost non-sorted tuple is located
    // to the left of the folded array, or when it does not exist at all (in this
    // case we are dealing with a sorted array);

    // is it possible to think of something more optimized? I don't think so.

    // does not work
    assert_eq!(rotation_index(&vec![4, 5, 6, 6, 4, 4]), 1);
    assert_eq!(minimum_in_rotated_array(2, &vec![4, 5, 6, 6, 4, 4]), 4);
    // assert_eq!(manual::rotation_index(&vec![3, 1, 3]), 1);
}
