struct Solution {}
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {

        // if we have enough steps to perform full binary search, of
        // course we can do it. But take logarith of N + 1, as even if
        // we have 1 floor as problem size, we still need to check it.
        let log2_i: i32 = ((n + 1) as f32).log2().ceil() as i32;
        if k >= log2_i {
            return log2_i;
        }

        // dp[N][K]
        let mut dp : Vec<Vec<i32>> = vec![];
        for _i in 0..=n as usize {
            dp.push([0].repeat((n + 1) as usize));
        }

        // When we have only one egg, it doesn't make sense to do
        // anything other than gradually trying each floor, starting from
        // the lowest one and moving up one floor for as long as the egg
        // does not crack.
        for i in 1..=n as usize {
            dp[i as usize][1] = i as i32;
        }

        // Need this as induction base.
        for i in 1..=n as usize {
            dp[1][i as usize] = 1;
        }

        // println!("{:?}", dp);

        for i in 2..=n as usize {
            // println!("i : {}", i);
            // the following loop is optimizied - we only need to
            // calculate the best answer till we have enough {k} to
            // perform full binary search. Than changes the complexity
            // of inner loop from O(N) to O(log2(N)). Therefore the
            // complexity is O(N * N * log2(N)).
            for k in 2..=n as usize {
                // println!("{} floors and {} eggs", i, k);
                let log2_i : usize = ((i + 1) as f32).log2().ceil()
                    as usize;
                // println!("i {} and log {}", i, log2_i);
                if k > log2_i {
                    // println!("reusing the solution {}", dp[i][log2_i]);
                    dp[i][k] = dp[i][log2_i];
                    continue;
                }

                let mut ans: i32 = i32::max_value();
                // A small optimization - it always makes sense to start
                // from the first floor and stop in the middle.
                let upper_bound: usize;
                if i % 2 == 0 {
                    upper_bound = i / 2;
                } else {
                    upper_bound = i / 2 + 1;
                }
                for j in 1..=upper_bound as usize {
                    // println!("dropping from floor: {}", j);
                    // The left part is already calculated as it involves
                    // smaller values of N and K. The right one can be
                    // calculated in one move using existing {dp} values.
                    let cracks: i32 = dp[j - 1][k - 1];
                    // println!("cracks with cost {}", cracks);
                    // println!("cost from dp[{}][{}]", j - 1, k - 1);
                    let survives: i32 = dp[i - j][k];
                    // println!("survives with cost {}", survives);
                    // println!("cost from dp[{}][{}]", i - j, k);
                    let worst_cost: i32 = std::cmp::max(cracks, survives);
                    if worst_cost < ans {
                        ans = worst_cost;
                    }
                }
                dp[i][k] = ans + 1;
            }
        }
        // println!("{:?}", dp);
        dp[n as usize][k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lc1() {
        assert_eq!(
            Solution::super_egg_drop(2, 9),
            4
        );
    }

    #[test]
    fn lc2() {
        assert_eq!(
            Solution::super_egg_drop(1, 2),
            2
        );
    }
}

fn main() {
/*
    assert_eq!(
        Solution::super_egg_drop(2, 9),
        4
    );
*/

    assert_eq!(
        Solution::super_egg_drop(4, 2000),
        4
    );

}
