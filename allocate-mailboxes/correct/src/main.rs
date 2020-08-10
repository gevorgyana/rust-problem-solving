fn main() {}

/* 0 3 7 10
 *
 * 1 4 6 7
 *
 * cost = 3 + 1 + 2 * 2 = 8
 *
 * 7 4 6 1
 * cost =
 */

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

        let left_prefix
            =
            Self::prefix(
                &houses,
                left_rule)
            ;

        println!("{:?}", left_prefix);

        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=k {
            dp.push([i32::max_value()].repeat(houses.len() + 1));
        }

        /// Calcualtes the amount of nodes to the right from {i} to {j}
        /// by manipulating {view} in O(1) time.
        /// Note that we cannot implement a similar function
        /// {left_starting_from} by flipping indices - see README.md.
        /// left_from_j_to_i != right_from_i_to_j
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
            let place_at = i / 2;
            let left_cost = left_prefix[place_at];
            let right_cost
                = right_from_till(place_at, i, &left_prefix, &houses);
            println!("i {}", i);
            println!("place_at {}", place_at);
            println!("left_cost {}", left_cost);
            println!("right_cost {}", right_cost);
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
                    let place_at = (rl + offset);
                    let left_cost
                        = right_from_till(rl, rl + offset,
                                          &left_prefix, &houses);
                    let right_cost
                        = right_from_till(rl + offset, nc,
                                          &left_prefix, &houses);

                    /*
                    println!("left_cost = {}", left_cost);
                    println!("right_cost = {}", right_cost);
                     */

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

        println!("{:?}", dp);

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
    //#[test]
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
