struct Solution {}
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        // dp[N][K] - I like N first.
        let mut dp : Vec<Vec<i32>> = vec![];
        for i in 0..n + 1 {
            dp.push([0].repeat(n as usize + 1));
        }
        // calculate first row
        for i in 1..n + 1 {
            dp[1][i as usize] = i;
        }
        for i in 2..n + 1 {
            for k in 1..i+1 {
                // We are calculating the answer for dropping an egg from
                // {j}-th floor when the building has {i} floors when we
                // have {k} eggs.
                let ans: i32 = i32::max_value();
                // Find the floor which yields the minimal difference
                // between the costs of solving the upper and lower
                // subproblems. A small optimization - it always makes
                // sense to start from the first floor and stop in the
                // middle.
                for j in 1..n / 2 + 1 {
                    // The left part is already calculated as it involves
                    // a smaller value of K. The right one can be
                    // calculated in one move from the existing results.
                    let cracks: i32 = dp[i - 1][k - 1];
                    let survives: i32 = dp[n - i][k];
                    if i32::abs(cracks - survives) < ans {
                        ans = i32::abs(cracks - survives);
                    }
                }
                dp[i][k] = ans;
            }
        }
        1
    }
}

fn main() {
    println!("Hello, world!");
}
