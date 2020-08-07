struct Solution {}
impl Solution {

    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {

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

        let v: Vec<&i32> = houses.iter().collect();
        let r: Vec<&i32> = houses.iter().rev().collect();

        let left_cost: Vec<i32> = scan_cost(left_cost, v);
        let right_cost: Vec<i32> = scan_cost(right_cost, r);

        println!("{:?}", left_cost);
        println!("{:?}", right_cost);

        1
    }
}

fn main() {
    Solution::min_distance(vec![1, 2], 2);
    println!("----");
    Solution::min_distance(vec![1, 2, 5], 2);
}
