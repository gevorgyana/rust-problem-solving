#[test]
fn main() {
    // testing find_min fn

    /*
    let vec = vec!(1, 2, 3, 4, 5);
    assert_eq!(find_min(vec), 1);

    let vec = vec!(5, 1, 2, 3, 4);
    assert_eq!(find_min(vec), 1);

    let vec = vec!(4, 5, 1, 2, 3);
    assert_eq!(find_min(vec), 1);

    let vec = vec!(3, 4, 5, 1, 2);
    assert_eq!(find_min(vec), 1);

    let vec = vec!(2, 3, 4, 5, 1);
    assert_eq!(find_min(vec), 1);

    let vec = vec!(1, 3, 5);
    assert_eq!(find_min(vec), 1);

    let vec = vec!(2, 2, 2, 0, 1);
    assert_eq!(find_min(vec), 0);

    let vec = vec!(3, 1, 3);
    assert_eq!(find_min(vec), 1);
    */

    let vec = vec!(3,3,3,3,3,3,3,3,1,3);
    assert_eq!(find_min(vec), 1);
}

/*
was looking for max value so that the min value is right next to it;
looking for the min value now;
is the function unimodal with respect to min?
    c
  b
a      e
     d

d e cannot reach higher than a

*/

pub fn find_min(nums: Vec<i32>) -> i32 {
    let initial_slice: &[i32] = &nums;
    let mut vec: &[i32] = &nums;
    let mut offset : usize = 0;
    let mut pos: usize = 0;
    let mut min: i32 = i32::max_value();
    println!("Started");
    loop {
        if vec.len() < 3 {
            println!("offset is {}", offset);
            if vec.len() == 1 {
                println!("One");
                pos = offset;
                min = std::cmp::min(min, vec[0]);
                break;
            } else {
                if vec[0] < vec[1] {
                    println!("Two");
                    pos = offset + 1;
                    min = std::cmp::min(min, vec[0]);
                    break;
                } else {
                    println!("Three");
                    pos = offset;
                    min = std::cmp::min(min, vec[1]);
                    break;
                }
            }
        };
        let m1: usize = vec.len() / 3 - 1;
        let m2: usize = vec.len() - vec.len() / 3;
        println!("{:?}", vec);
        println!("Comparing {} ({}) to {} ({})", vec[m1], m1, vec[m2], m2);

        // find min here
        min = std::cmp::min(min,
                            std::cmp::min(vec[m1], vec[m2]));

        match vec[m1].cmp(&vec[m2]) {
            std::cmp::Ordering::Less => {
                println!("Less");
                vec = &vec[m1 + 1..vec.len()];
                offset += m1 + 1;
            },
            std::cmp::Ordering::Equal => {
                println!("Equal");
                vec = &vec[m1 + 1..m2];
                offset += m1 + 1;
            },
            std::cmp::Ordering::Greater => {
                println!("Greater");
                vec = &vec[0..m2];
            },
        }
    }

    println!("Survived with pos {}", pos);

    min = std::cmp::min(nums[(pos + 1) % nums.len()],
                        min);

    println!("Next check");

    let mut vec = initial_slice;
    loop {
        if vec.len() < 3 {
            if vec.len() == 1 {
                println!("One");
                min = std::cmp::min(min, vec[0]);
            } else {
                if vec[0] < vec[1] {
                    println!("Two");
                    min = std::cmp::min(min, vec[0]);
                } else {
                    println!("Three");
                    min = std::cmp::min(min, vec[1]);
                }
            }
            break;
        };
        let m1: usize = vec.len() / 3 - 1;
        let m2: usize = vec.len() - vec.len() / 3;
        println!("{:?}", vec);
        println!("Comparing {} ({}) to {} ({})", vec[m1], m1, vec[m2], m2);

        min = std::cmp::min(min,
                            std::cmp::min(vec[m1], vec[m2]));

        match vec[m1].cmp(&vec[m2]) {
            std::cmp::Ordering::Less => {
                println!("Less");
                vec = &vec[0..m2];
            },
            std::cmp::Ordering::Equal => {
                println!("Equal");
                vec = &vec[m1 + 1..m2];
            },
            std::cmp::Ordering::Greater => {
                println!("Greater");
                vec = &vec[m1 + 1..vec.len()];
            },
        }
    }

    min
}
