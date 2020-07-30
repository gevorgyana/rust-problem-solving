struct Solution {}
impl Solution {
    /*
     * a|b|c|d    S
     * +       a  T
     * --------
     *     +   c
     */
    pub fn num_distinct(s: String, t: String) -> i32 {
        println!("S: {}", s);
        println!("T: {}", t);

        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..t.len() {
            dp.push([0].repeat(s.len()));
        }

        // first row
        for (i, val) in t.chars().enumerate() {
            for (j, val2) in s.chars().enumerate() {

                if val2 == val {
                    println!("eq s:{} == t:{}", j, i);
                    println!("from {} to {}", j, s.len() - 1);
                    let mut mul = 1;
                    if i > 0 {
                        mul *= dp[i - 1][j];
                    }
                    println!("adding this {}", mul);
                    for k in j..s.len() {
                        dp[i][k] += mul;
                    }
                }
            }
        }

        println!("{:?}", dp);

        1
    }
}

fn main() {
    let f: i32 = Solution::
    num_distinct(String::from("abcdac"), String::from("ac"));
}
