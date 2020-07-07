#[test]
fn main() {
    // testing find_min fn
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
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let initial_slice: &[i32] = &nums;

    let mut vec: &[i32] = &nums;
    let mut offset : usize = 0;
    let mut pos: usize = 0;

    loop {

        if vec.len() < 3 {
            println!("offset is {}", offset);
            if vec.len() == 1 {
                println!("One");
                pos = offset;
                break;
            } else {
                if vec[0] < vec[1] {
                    println!("Two");
                    pos = offset + 1;
                    break;
                } else {
                    println!("Three");
                    pos = offset;
                    break;
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

    println!("Survived with pos {}", pos);
    nums[(pos + 1) % nums.len()]
}
