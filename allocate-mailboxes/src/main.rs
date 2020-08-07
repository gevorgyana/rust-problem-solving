struct Solution {}
impl Solution {

    pub fn min_distance(houses: Vec<i32>) -> i32 {
        if houses.len() == 1 {
            return 0;
        }
        println!("{:?}", houses);
        let left_cost = | cost: &mut i32, x: (usize, &[&i32]) | {
            let diff: i32 = x.1[1] - x.1[0];
            *cost += diff * (x.0 as i32 + 1);
            Some(*cost)
        };
        let right_cost = | cost: &mut i32, x: (usize, &[&i32]) | {
            let diff: i32 = x.1[0] - x.1[1];
            *cost += diff * (x.0 as i32 + 1);
            Some(*cost)
        };
        // accepts references
        fn scan_cost<F>(scan_rule: F, vec: Vec<&i32>) -> Vec<i32>
        where F : FnMut(&mut i32, (usize, &[&i32])) -> Option<i32> {
            std::iter::once(0)
                .chain(
                    vec
                        .windows(2)
                        .enumerate()
                        .scan(0,
                              scan_rule
                        )
                )
                .collect()
        };
        let left_cost: Vec<i32> = scan_cost(
            left_cost,
            houses.iter().collect::<Vec<&i32>>()
        );
        let right_cost: Vec<i32> = scan_cost(
            right_cost,
            houses.iter().rev().collect::<Vec<&i32>>()
        );

        println!("{:?}", left_cost);
        println!("{:?}", right_cost);

        let mut solution: (usize, i32) = (0, i32::max_value());
        for i in 0..left_cost.len() {
            let cost = left_cost[i] + right_cost[left_cost.len() - i - 1];
            println!("!cost = {}", cost);
            if std::cmp::min(cost, solution.1) == cost {
                solution.1 = cost;
                solution.0 = i;
            }
        }

        solution.0 as i32
    }
}

fn main() {
/*
    assert_eq!(
        Solution::min_distance(vec![1, 2]),
        1
    );
*/
    println!("----");

    assert_eq!(
        Solution::min_distance(vec![1, 2, 5]),
        1
    );

}
