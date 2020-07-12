// credits to this project which is licensed under APACHE-2
// https://github.com/alkis/superslice-rs
use std::cmp;

pub fn rotation_index(vec: &Vec<i32>) -> usize {
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
        } else {
            cmp::Ordering::Greater
        }
    };

    let f = compare_tuples;
    let s = cooked;
    let mut size = s.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let cmp = f(unsafe { s.get_unchecked(mid) });
        base = if cmp == std::cmp::Ordering::Less { mid } else { base };
        size -= half;
    }
    let cmp = f(unsafe { s.get_unchecked(base) });
    base + (cmp == std::cmp::Ordering::Less) as usize
}

fn minimum_in_rotated_array(rotation_point: i32, vec: &Vec<i32>) -> i32 {
    if rotation_point == 0 {
        vec[0]
    } else {
        std::cmp::min(vec[rotation_point as usize],
                  vec[vec.len() - rotation_point as usize])
    }
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let index = rotation_index(&nums);
    let answer = minimum_in_rotated_array(index as i32, &nums);
    answer
}
