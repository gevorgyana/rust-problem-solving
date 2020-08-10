fn main() {}

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

        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=k {
            dp.push([0].repeat(houses.len() + 1));
        }

        /// Calcualtes the amount of nodes to the right from {i} to {j}
        /// by manipulating {view} in O(1) time.
        /// Note that we could also implement a similar function
        /// {left_starting_from}, but it would do the same - so
        /// just shift the inidices and call this function instead.
        fn right_from_till(i: usize, j: usize,
                           left_prefix_view: &Vec<i32>,
                           houses_view: &Vec<i32>
        ) -> i32 {
            left_prefix_view[j]
                - left_prefix_view[i]
                - i as i32
                * (houses_view[j] - houses_view[i])
        }

        // the answer for k = 1
        for i in 0..houses.len() {
            println!("the last house {}", i);
            let place_at = i / 2;
            let left_cost = left_prefix[place_at];
            let right_cost
                = right_from_till(place_at, i, &left_prefix, &houses);
            println!("right cost {}", right_cost);
            dp[1][i] = left_cost + right_cost;
            println!("dp = {}", dp[1][i]);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lc1() {
        assert_eq!(
            Solution::min_distance(vec![1, 4, 8, 10, 20], 3),
            5
        );
    }
}
