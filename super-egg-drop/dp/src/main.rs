struct Solution {}
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {

        /*
        // if we have enough steps to perform full binary search, of
        // course we can do it.
        let log2_i: i32 = (n as f32).log2().ceil() as i32;
        if k >= log2_i {
            return log2_i;
        }
         */

        // dp[N][K] - I like N first. One layer of values is not used,
        // but it helps us use 1-based indexing.
        let mut dp : Vec<Vec<i32>> = vec![];
        for _i in 0..=n as usize {
            dp.push([0].repeat((n + 1) as usize));
        }

        /*
        println!("{:?}", dp);
        panic!("done for now");
         */

        // When we have only one egg, it doesn't make sense to do
        // anything other than gradually trying each floor, starting from
        // the lowest one and moving up one floor for as long as the egg
        // does not crack.
        for i in 1..=n as usize {
            dp[i as usize][1] = i as i32;
        }

        for i in 1..=n as usize {
            dp[1][i as usize] = 1;
        }

        println!("{:?}", dp);

        for i in 2..=n as usize {
            if i > 2 {break;}
            // todo optimize till i or log2(i)
            for k in 2..=n as usize {
                println!("{} floors and {} eggs", i, k);
                let mut ans: i32 = i32::max_value();
                // A small optimization - it always makes sense to start
                // from the first floor and stop in the middle.
                for j in 1..=i as usize {
                    println!("dropping from floor: {}", j);
                    // The left part is already calculated as it involves
                    // smaller values of N and K. The right one can be
                    // calculated in one move using existing {dp} values.
                    let mut cracks: i32 = 0;
                    cracks = dp[j - 1][k - 1];
                    println!("cracks with cost {}", cracks);
                    println!("cost from dp[{}][{}]", j - 1, k - 1);
                    let survives: i32 = dp[i - j][k];
                    println!("survives with cost {}", survives);
                    println!("cost from dp[{}][{}]", i - j, k);

                    let worst_cost: i32 = std::cmp::max(cracks, survives);
                    if worst_cost < ans {
                        ans = worst_cost;
                    }
                }
                dp[i][k] = ans;
            }
        }
        println!("{:?}", dp);
        dp[n as usize][k as usize]
    }
}

fn main() {
    Solution::super_egg_drop(2, 9);
}
