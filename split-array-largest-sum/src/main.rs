struct Solution {}
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        // As usual, create (N + 1) x (M + 1) values for easier indexing
        // and holding the results of invalid queries.
        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=m {
            // induction base
            if i == 1 {
                dp.push(
                    std::iter::once(0).chain(
                        nums.iter().scan(0, |sum, &cur| {
                            *sum += cur;
                            Some(*sum)
                        }))
                        .collect()
                );
            } else {
                dp.push([0].repeat(nums.len() + 1));
            }
        }

        for i in 2..=m as usize {
            println!("m = {}", i);
            for j in i as usize..=nums.len() {
                println!("array elements from {} to {} are used",
                         i, j
                );
                dp[i][j] = i32::max_value();

                // sliding window size
                for k in i..=j {
                    let window_size = k - i + 1;
                    println!("window size = {}", k - i + 1);
                    let range_size = j - i + 1;
                    println!("range size {}", range_size);

                    // move the window
                    for s in 0..=(range_size - window_size) {
                        println!("offset s {}", s);
                        // update the answer for {i} subarrays and {j}
                        // as the rightmost available index of the array,
                        // using {k} and {s} as the location of the
                        // subarray being checked.
                        let prefix_sum_rightmost
                            = dp[1][i + s + window_size - 1]
                            - dp[1][i + s - 1];
                        println!("prefix_sum_rightmost {}",
                                 prefix_sum_rightmost);

                        println!("answer for the remaining subproblem {}",
                                 dp[i - 1][i + s - 1]
                        );

                        dp[i][j] = std::cmp::min(
                            dp[i][j],
                            std::cmp::max(
                                dp[i - 1][i + s - 1],
                                prefix_sum_rightmost
                            )
                        );
                    }
                }
            }
        }

        println!("{:?}", dp);

        1
    }
}

fn main() {
    /*
    Solution::split_array(
        vec![1, 2, 3], 2
    );
     */
    Solution::split_array(
        vec![7, 2, 5, 10, 8], 2
    );
}
