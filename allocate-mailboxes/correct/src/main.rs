fn main() {}

struct Solution {}

impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {

        houses.sort();
        println!("houses {:?}", houses);

        let left_rule = |scanned: &mut i32, x: (usize, &[i32])| {
            let diff = x.1[1] - x.1[0];
            *scanned += diff * (x.0 + 1) as i32;
            Some(*scanned)
        };

        let right_rule = |scanned: &mut i32, x: (usize, &[i32])| {
            let diff = x.1[0] - x.1[1];
            *scanned += diff * (x.0 + 1) as i32;
            Some(*scanned)
        };

        let left_prefix
            =
            Self::prefix(
                &houses,
                left_rule)
            ;

        let right_prefix
            =
            Self::prefix(
                &houses
                    .clone()
                    .into_iter()
                    .rev()
                    .collect(),
                right_rule)
            ;

        // println!("{:?}", left_prefix);
        // println!("{:?}", right_prefix);

        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=k {
            dp.push([i32::max_value()].repeat(houses.len() + 1));
        }

        /// Produces the sum of all distances from {j} to every {k}
        /// such that {k} >= {i} by manipulating {view} in O(1) time.
        ///
        /// Note that we cannot implement a similar function
        /// {right_starting_from} by flipping indices - see README.md.
        /// i < j
        fn left_from_till(i: usize, j: usize,
                           left_prefix_view: &Vec<i32>,
                           houses_view: &Vec<i32>
        ) -> i32 {
            left_prefix_view[j]
                - left_prefix_view[i]
                - i as i32
                * (houses_view[j] - houses_view[i])
        }

        /// Produces the sum of the all distances from {i}
        /// to every {k} such that {k} <= {j}; i < j
        fn right_from_till(i: usize, j: usize,
                           right_prefix_view: &Vec<i32>,
                           houses_view: &Vec<i32>
        ) -> i32 {
            let n = houses_view.len();

            /*
            println!("n - 1 - i {} | n - 1 - j {}",
                     n - 1 - i,
                     n - 1 - j
            );
            println!("right[n - 1 - i] {}", right_prefix_view[n - 1 - i]);
            println!("right[n - 1 - j] {}", right_prefix_view[n - 1 - j]);
            println!("n - 1 - j {}", (n - 1 - j));
            println!("h[n - 1 - j] - h[n - 1 - i] {}",
                     (houses_view[n - 1 - j] - houses_view[n - 1 - i])
            );
             */

            right_prefix_view[n - 1 - i]
                - right_prefix_view[n - 1 - j]
                - (n - 1 - j) as i32
                * (houses_view[j] - houses_view[i])
        }

        // the answer for k = 1
        for i in 0..houses.len() {
            // println!("i {}", i);
            let place_at = i / 2;
            // println!("place_at {}", place_at);
            let left_cost = left_prefix[place_at];
            let right_cost = right_from_till(i / 2, i,
                                             &right_prefix, &houses);
            // println!("left_cost {}", left_cost);
            // println!("right_cost {}", right_cost);
            dp[1][i + 1] = left_cost + right_cost;
        }

        for kc in 2..=k as usize {
            // if kc == 3 { break; }
            // println!("kc = {}", kc);
            for nc in kc - 1..houses.len() {
                // println!("nc = {}", nc);
                // the beginning of the rightmost range
                for rl in kc - 1..=nc {
                    // println!("rl = {}", rl);
                    // rl..nc is the rightmost range
                    let offset = (nc - rl) / 2;
                    let place_at = rl + offset;
                    let left_cost
                        = left_from_till(rl, rl + offset,
                                          &left_prefix, &houses);
                    let right_cost
                        = right_from_till(rl + offset, nc,
                                          &right_prefix, &houses);

                    // println!("left_cost = {}", left_cost);
                    // println!("right_cost = {}", right_cost);

                    dp[kc][nc + 1] = std::cmp::min(dp[kc][nc + 1],
                                                   left_cost + right_cost
                                                   + dp[kc - 1][rl]
                    );

                    /*
                    println!("dp[kc][nc + 1] updated with {}",
                             left_cost + right_cost
                             + dp[kc - 1][rl]
                    );
                     */
                }
            }
        }

        // println!("{:?}", dp);
        dp[k as usize][houses.len()]
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

    #[test]
    fn lc2() {
        assert_eq!(
            Solution::min_distance(vec![7, 4, 6, 1], 1),
            8
        );
    }
}
