fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {

    let definitely_taken : i32 = customers.iter()
        .enumerate()
        .fold(0, |sum, (id, val)| {
            if grumpy[id] == 0 {
                sum + val
            } else {
                sum
            }
        });

    let best_gain : i32 = customers
        .iter()
        .enumerate()
        .collect::<Vec::<(usize, &i32)>>()
        .windows(x as usize)
        .max_by(|lhs, rhs| {
            let take_whole_window = |win: &[(usize, &i32)]| {
                win
                    .iter()
                    .fold(0, |sum, (id, val)| {
                        sum + *val
                    })
            };

            let lhsr = take_whole_window(*lhs);
            let rhsr = take_whole_window(*rhs);

            if std::cmp::max(lhsr, rhsr) == rhsr { std::cmp::Ordering::Less }
            else { std::cmp::Ordering::Greater }
        })
        .unwrap()
        .iter()
        .fold(0, |sum, (id, val)| {
            sum + *val
        });

    definitely_taken + best_gain
}

fn main() {
    assert_eq!(
        3, max_satisfied(vec![1, 2, 3], vec![1, 1, 1], 1)
    );
}
