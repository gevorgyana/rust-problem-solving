struct Solution {}
impl Solution {

    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {

        println!("houses {:?}", houses);

        let left_rule = |scanned: &mut i32, x: (usize, &[i32])| {
            let diff = x.1[1] - x.1[0];
            *scanned += diff * (x.0 + 1) as i32;
            Some(*scanned)
        };

        let left_prefix
            =
            Self::prefix(
                &houses,
                left_rule)
            ;

        println!("{:?}", left_prefix);

        // calculate for houses their right prices
        // right price is left price with a shift.
        for i in 0..houses.len() {
            let left_cost = left_prefix[i];
            let right_cost
                = left_prefix[houses.len() - 1]
                - left_prefix[i]
                - i as i32 * (houses[houses.len() - 1] - houses[i]);
            println!("left {}, right {}", left_cost, right_cost);
        }

        1
    }

    fn prefix<F> (vec: &Vec<i32>, rule: F) -> Vec<i32>
    where F: FnMut(&mut i32, (usize, &[i32])) -> Option<i32>
    {
        std::iter::once(0)
            .chain(
                vec
                    .windows(2)
                    .enumerate()
                    .scan(
                        0,
                        rule
                    )
            )
            .collect()
    }
}

fn main() {
    Solution::min_distance(vec![1, 2, 3, 4, 5], 1);
}
