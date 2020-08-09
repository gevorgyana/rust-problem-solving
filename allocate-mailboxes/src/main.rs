// if interested, see the history of this file to understand how the
// solution works step by step

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

                        // println!("left cost {}", left_cost);
                        // println!("right cost {}", right_cost);
                    }
            }

            // check the last house - this works when N = 1
            /*
            println!("checking the last house, its index is {}",
                     hm_houses_avail - 1
            );
            println!("coordinate of the last hosue {}",
                     houses[hm_houses_avail - 1]);
             */
            let left_cost
                = left_prefix[hm_houses_avail - 1];
            let right_cost = 0;
            // println!("left cost {}", left_cost);
            // println!("right cost {}", right_cost);

            ans = std::cmp::min(
                ans,
                (left_cost - right_cost).abs()
            );
            // println!("?The answer {}", ans);
            dp[1][hm_houses_avail] = ans;
        }

        println!("{:?}", dp);

        // 2 mailboxes and calculate just the needed answer now -
        // calculate only for the last one.
        for left_part_sz in 1..houses.len() {
            println!("");
            println!("size of the left part {}", left_part_sz);
            for hm_houses_avail in 1..=houses.len() - left_part_sz {
                println!("# houses available {}", hm_houses_avail);
                let mut ans = i32::max_value();

                for window_start in 0..hm_houses_avail - 1 {

                    println!("window start {}, end {}",
                             window_start + left_part_sz,
                             window_start + 1 + left_part_sz);
                    println!("house start {}, end {}",
                             houses[window_start + left_part_sz],
                             houses[window_start + 1 + left_part_sz]);

                    /*
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

                            println!("left cost {}", left_cost);
                            println!("right cost {}", right_cost);
                        }
                     */
                }

                println!("checking the last house, its index is {}",
                         hm_houses_avail - 1 + left_part_sz
                );
                println!("coordinate of the last house {}",
                         houses[hm_houses_avail - 1 + left_part_sz]);

                let left_cost
                    = left_prefix[hm_houses_avail - 1 + left_part_sz]
                    - left_prefix[left_part_sz]
                    - (left_part_sz) as i32
                    * (houses[hm_houses_avail - 1 + left_part_sz]
                       - houses[left_part_sz]
                       );

                let right_cost = 0;
                println!("left cost {}", left_cost);
                println!("right cost {}", right_cost);

                ans = std::cmp::min(
                    ans,
                    (left_cost - right_cost).abs()
                );

                println!("?The answer {}", ans);
                dp[2][hm_houses_avail] = ans;
            }
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
    Solution::min_distance(vec![1, 2, 3, 4, 5], 2);
}
