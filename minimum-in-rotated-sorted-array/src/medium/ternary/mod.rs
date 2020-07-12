#[test]
fn main() {

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

    let vec = vec!(2, 3, 4, 5, 6, 1);
    assert_eq!(find_min(vec), 1);

}

pub fn find_min(nums: Vec<i32>) -> i32 {
    // we need a slice, not &Vec
    let mut vec: &[i32] = &nums;
    let mut offset : usize = 0;
    let mut min: i32 = i32::max_value();

    println!("Started");

    let mut m1: usize = 0;
    let mut m2: usize = vec.len() - 1;
    let mut first: bool = true;

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

        if first == false {
            m1 = vec.len() / 3 - 1;
            m2 = vec.len() - vec.len() / 3;
        } else {
            first = false;
        }

        println!("{:?}", vec);
        println!("Comparing {} ({}) to {} ({})", vec[m1], m1, vec[m2], m2);

        match vec[m1].cmp(&vec[m2]) {
            std::cmp::Ordering::Less => {
                println!("Less");
                vec = &vec[0..m2];
            },
            std::cmp::Ordering::Greater => {
                println!("Greater");
                vec = &vec[m1 + 1..vec.len()];
            },
            // there are no equal elements in the array
            _ => (),
        }
    }

    min
}
