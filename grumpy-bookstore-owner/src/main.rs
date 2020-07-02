fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    definitely_taken(&customers, &grumpy) + best_gain(&customers, &grumpy, x)
}

fn definitely_taken(customers: &Vec<i32>, grumpy: &Vec<i32>) -> i32 {

    customers.iter()
        .enumerate()
        .fold(0, |sum, (id, val)| {
            if grumpy[id] == 0 {
                sum + val
            } else {
                sum
            }
        })
}

fn best_gain(customers: &Vec<i32>, grumpy: &Vec<i32>, x: i32) -> i32 {

    let gain = |sum : i32, (id, val): &(usize, &i32)| {
        if grumpy[*id] == 1 {
            sum + *val
        } else {
            sum
        }
    };

    customers
        .iter()
        .enumerate()
        .collect::<Vec::<(usize, &i32)>>()
        .windows(x as usize)
        .max_by(|lhs, rhs| {
            let window_gain = |win: &[(usize, &i32)]| {
                win
                    .iter()
                    .fold(0, gain)
            };
            let lhsr = window_gain(*lhs);
            let rhsr = window_gain(*rhs);
            if std::cmp::max(lhsr, rhsr) == rhsr { std::cmp::Ordering::Less }
            else { std::cmp::Ordering::Greater }
        })
        .unwrap()
        .iter()
        .fold(0, gain)
}

fn main() {

    // completed tests
    assert_eq!(
        3, max_satisfied(vec![1, 2, 3], vec![1, 1, 1], 1)
    );
    assert_eq!(
        4, max_satisfied(vec![1, 2, 3], vec![0, 1, 1], 1)
    );
    assert_eq!(
        5, max_satisfied(vec![1, 2, 3], vec![1, 0, 1], 1)
    );
    assert_eq!(
        5, max_satisfied(vec![1, 2, 3], vec![1, 1, 0], 1)
    );
    assert_eq!(
        6, max_satisfied(vec![1, 2, 3], vec![0, 0, 1], 1)
    );
    assert_eq!(
        6, max_satisfied(vec![1, 2, 3], vec![1, 0, 0], 1)
    );
    assert_eq!(
        6, max_satisfied(vec![1, 2, 3], vec![0, 0, 0], 1)
    );
}
