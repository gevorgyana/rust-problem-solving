/*
        e
  b c d
a         f g
---
        e
    c d
a b      f g
---
a b c d e f g
---
        e
  b c d
a           g
          f
*/

/// returns the position of the maximum element
fn ternary_search(mut vec: &[i32]) -> usize {
    println!("Initial {:?}", vec);
    let mut offset : usize = 0;
    loop {

        if vec.len() < 3 {
            println!("offset is {}", offset);
            if vec.len() == 1 {
                return offset
            } else {
                if vec[0] < vec[1] {
                    return offset + 1
                } else {
                    return offset
                }
            }
        };

        let m1: usize = vec.len() / 3 - 1;
        let m2: usize = vec.len() - vec.len() / 3;

        match vec[m1].cmp(&vec[m2]) {
            std::cmp::Ordering::Less => {
                println!("Less");
                vec = &vec[m1 + 1..vec.len()];
                println!("{:?}", vec);
                offset += m1 + 1;
            },
            std::cmp::Ordering::Equal => {
                println!("Equal");
                vec = &vec[m1 + 1..m2];
                println!("{:?}", vec);
                offset += m1 + 1;
            },
            std::cmp::Ordering::Greater => {
                println!("Greater");
                vec = &vec[0..m2];
                println!("{:?}", vec);
            },
        }
    }
}

fn solution(vec: &[i32]) -> i32 {
    // find the position of the greatest element, this will be the offset that you
    // need to apply to the array to find its first element, which is going to be the
    // minimum asked in the problem statement
    let pos = ternary_search(&vec);
    vec[(pos + 1) % vec.len()]
}

#[test]
fn main() {
    // general testing of ternary search
    assert_eq!(ternary_search(&vec!(1, 1, 3, 1)), 2);
    assert_eq!(ternary_search(&vec!(1, 2, 3, 4, 5, 8, 1)), 5);
    /* just for logs on how the search advances
    ternary_search(&vec!(1, 2, 3, 4));
    ternary_search(&vec!(4, 3, 2, 1));
     */

    // testing ternary_search to find maximum in rotated arrays
    let vec = vec!(1, 2, 3, 4, 5);
    assert_eq!(ternary_search(&vec), 4);
    let vec = vec!(5, 1, 2, 3, 4);
    assert_eq!(ternary_search(&vec), 0);
    let vec = vec!(4, 5, 1, 2, 3);
    assert_eq!(ternary_search(&vec), 1);
    let vec = vec!(3, 4, 5, 1, 2);
    assert_eq!(ternary_search(&vec), 2);
    let vec = vec!(2, 3, 4, 5, 1);
    assert_eq!(ternary_search(&vec), 3);

    // testing on some vectors with repeated values
    let vec = vec!(2, 3, 5, 5, 1);
    assert_eq!(ternary_search(&vec), 2);

    // testing solution fn
    let vec = vec!(1, 2, 3, 4, 5);
    assert_eq!(solution(&vec), 1);
    let vec = vec!(5, 1, 2, 3, 4);
    assert_eq!(solution(&vec), 1);
    let vec = vec!(4, 5, 1, 2, 3);
    assert_eq!(solution(&vec), 1);
    let vec = vec!(3, 4, 5, 1, 2);
    assert_eq!(solution(&vec), 1);
    let vec = vec!(2, 3, 4, 5, 1);
    assert_eq!(solution(&vec), 1);
}
