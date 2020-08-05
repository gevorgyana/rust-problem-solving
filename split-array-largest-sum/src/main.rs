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

        // The answer for N = * and M = 1 is already calculated.

        for i in 2..=nums.len() {
            println!("size N = {}", i);
            for j in 2..=m as usize {
                println!("divs MM = {}", j);
                dp[j][i] = i32::max_value();
                // use linear search to update the answer
                // some of the answers will be invalid! those where
                // the following loop will not be executed - those that
                // have {j} > {i}, meaning that the value of M is greater
                // than N - in which case it is impossible to give an
                // answer.
                for k in j as usize..=i {
                    println!("begin the rightmost array at {}",
                             k);
                    let rightmost_prefix_sum
                        = dp[1][i] - dp[1][k - 1];
                    let leftmost_answer
                        = dp[j - 1][k - 1];
                    println!("rightmost prefix sum {}; leftmost ans {}",
                             rightmost_prefix_sum,
                             leftmost_answer
                    );

                    dp[j][i] = std::cmp::min(
                        dp[j][i],
                        std::cmp::max(
                            leftmost_answer,
                            rightmost_prefix_sum
                        )
                    );
                }
            }
        }

        println!("{:?}", dp);
        println!("orignal {:?}", nums);

        dp[m as usize][nums.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lc1() {
        assert_eq!(
            Solution::split_array(
                vec![7, 2, 5, 10, 8], 2
            ),
            18
        );
    }

    #[test]
    fn lc2() {
        assert_eq!(
            Solution::split_array(
                vec![7, 2, 5, 10, 8], 3
            ),
            14
        );
    }
}

fn main() {}
