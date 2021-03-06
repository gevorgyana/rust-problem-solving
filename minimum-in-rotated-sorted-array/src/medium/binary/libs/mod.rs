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
}
