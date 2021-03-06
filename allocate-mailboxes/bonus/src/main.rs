// if interested, see the history of this file to understand how the
// solution works step by step

struct Solution {}
impl Solution {

    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {

        // println!("houses {:?}", houses);

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

        // println!("{:?}", left_prefix);

        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=k {
            dp.push([0].repeat(houses.len() + 1));
        }

        for hm_houses_avail in 1..=houses.len() {
            // println!("# houses {}", hm_houses_avail);
            let mut ans = i32::max_value();

            for window_start in 0..hm_houses_avail - 1 {
                /*
                println!("window start {}, end {}",
                         window_start, window_start + 1);
                println!("house start {}, end {}",
                         houses[window_start],
                         houses[window_start + 1]);
                 */

                for checked in houses[window_start]..
                    houses[window_start + 1] {

                        let left_cost
                            = left_prefix[window_start]
                            + (checked - houses[window_start]);

                        let right_cost
                            =
                            left_prefix[hm_houses_avail - 1]
                            -
                            left_prefix[window_start + 1]
                            -
                            (window_start as i32 + 1)
                            *
                            (houses[hm_houses_avail - 1]
                             - houses[window_start + 1])
                            +
                            (hm_houses_avail as i32 - 1
                             - window_start as i32)
                            *
                            (houses[window_start + 1]
                             - checked)
                            ;

                        ans = std::cmp::min(
                            ans,
                            (left_cost - right_cost).abs()
                        );
                    }
            }
            let left_cost
                = left_prefix[hm_houses_avail - 1];
            let right_cost = 0;
            ans = std::cmp::min(
                ans,
                left_cost + right_cost
            );
            // println!("?The answer {}", ans);
            dp[1][hm_houses_avail] = ans;
        }

        println!("{:?}", dp);

        for kc in 2..=k {
            if kc == 3 {
                break;
            }

            for left_part_sz in 1..houses.len() {
                println!("");
                println!("size of the left part {}", left_part_sz);
                for hm_houses_avail in 1..=houses.len() - left_part_sz {
                    println!("# houses available {}", hm_houses_avail);
                    let mut ans = i32::max_value();
                    for window_start in 0..hm_houses_avail - 1 {
                        let window_start = window_start + left_part_sz;
                        let hm_houses_avail = hm_houses_avail + left_part_sz;
                        for checked in houses[window_start]..
                            houses[window_start + 1] {
                                let left_cost
                                    = left_prefix[window_start]
                                    - left_prefix[left_part_sz]
                                    - left_part_sz as i32
                                    * (houses[window_start]
                                       - houses[left_part_sz]
                                    )
                                    + (checked - houses[window_start]);

                                let left_prefix_this_rightmost_window
                                    = left_prefix[window_start + 1]
                                    - left_prefix[left_part_sz]
                                    - left_part_sz as i32
                                    * (houses[window_start + 1]
                                       - houses[left_part_sz])
                                    ;

                                let left_prefix_this_last_house
                                    = left_prefix[hm_houses_avail - 1]
                                    - left_prefix[left_part_sz]
                                    - left_part_sz as i32
                                    * (houses[hm_houses_avail - 1]
                                       - houses[left_part_sz])
                                    ;

                                let right_cost_window_1
                                    = left_prefix_this_last_house
                                    - left_prefix_this_rightmost_window
                                    - ((window_start as i32 + 1) - left_part_sz as i32)
                                    * (houses[hm_houses_avail - 1]
                                       - houses[window_start + 1]);

                                let additional_shift
                                    = (hm_houses_avail as i32 - 1
                                       - window_start as i32)
                                    * (houses[window_start + 1]
                                       - checked)
                                    ;

                                let right_cost
                                    = right_cost_window_1
                                    + additional_shift
                                    ;

                                ans = std::cmp::min(
                                    ans,
                                    (left_cost - right_cost).abs()
                                );

                                println!("???left cost {}", left_cost);
                                println!("!!!right cost {}", right_cost);
                            }
                    }

                    let left_cost
                        = left_prefix[hm_houses_avail - 1 + left_part_sz]
                        - left_prefix[left_part_sz]
                        - (left_part_sz) as i32
                        * (houses[hm_houses_avail - 1 + left_part_sz]
                           - houses[left_part_sz]
                        );

                    let right_cost = 0;

                    println!("the cost of solving k = {}, n = {} is {}",
                    1,
                    left_part_sz,
                    dp[1][left_part_sz]
                );

                    ans = std::cmp::min(
                        ans,
                        (left_cost - right_cost).abs()
                    );

                    dp[kc as usize][hm_houses_avail + left_part_sz]
                        = ans
                        + dp[kc as usize - 1][left_part_sz as usize]
                        ;
                }
            }
        }
        println!("{:?}", dp);
        dp[k as usize][houses.len() as usize]
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
    fn cs1() {
        assert_eq!(
            Solution::min_distance(vec![1, 2, 3, 4, 5], 2),
            4
        );
    }
}

fn main() {

}
