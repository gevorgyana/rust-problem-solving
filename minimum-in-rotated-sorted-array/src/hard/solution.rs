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

/// returns the position of the maximum
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

fn main() {
    assert_eq!(ternary_search(&vec!(1, 1, 3, 1)), 2);
    assert_eq!(ternary_search(&vec!(1, 2, 3, 4, 5, 8, 1)), 5);
    /* just for logs on how the search advances
    ternary_search(&vec!(1, 2, 3, 4));
    ternary_search(&vec!(4, 3, 2, 1));
     */
}
