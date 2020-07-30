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
        let mut ans: i32 = 0;

        // first row
        for (i, val) in t.chars().enumerate() {
            println!("iter t: {}", i);
            for (j, val2) in s.chars().enumerate() {
                println!("iter s: {}", j);
                // small optimization
                if j < i { continue; }
                if val2 == val {

                    let mut mul = 1;
                    if i > 0 && dp[i - 1][j] > 0 {
                        mul *= dp[i - 1][j];
                    }

                    if i == t.len() - 1 {
                        ans += mul;
                    }

                    for k in j+1..s.len() {
                        dp[i][k] += mul;
                    }
                }
            }
        }
        // println!("{:?}", dp);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {

        assert_eq!(Solution::num_distinct(
            String::from("rabbbit"),
            String::from("rabbit")),
                   3);

        assert_eq!(Solution::num_distinct(
            String::from("rabbbit"),
            String::from("rabit")),
                   3);
        assert_eq!(Solution::num_distinct(
            String::from("bbb"),
            String::from("bb")),
                   3);
        assert_eq!(Solution::num_distinct(
            String::from("b"),
            String::from("b")),
                   1);
    }
}

fn main() {
    assert_eq!(Solution::num_distinct(
        String::from("abcdac"),
        String::from("ac")),
               3);
}
