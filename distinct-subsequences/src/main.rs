struct Solution {}
impl Solution {
    /*
     * a|b|c|d    S
     * +       a  T
     * --------
     *     +   c
     */
    pub fn num_distinct(s: String, t: String) -> i32 {
        //println!("S: {}", s);
        //println!("T: {}", t);

        let mut dp: Vec<Vec<i32>> = vec![];
        for _i in 0..t.len() - 1 {
            dp.push([0].repeat(s.len()));
        }
        let mut ans: i32 = 0;

        if s.len() == 0 {
            return 0;
        }

        // dp works for length > 1
        if t.len() == 1 {
            for (_i, val) in s.chars().enumerate() {
                if val == t.chars().nth(0).unwrap() {
                    ans += 1;
                }
            }
            return ans;
        }

        for (i, val) in t.chars().enumerate() {
            for (j, val2) in s.chars().enumerate() {
                if j < i {continue;}
                if val2 == val {
                    //println!("t: {}, s: {}", i, j);
                    let mut mul = 1;
                    if i > 0 && dp[i - 1][j] > 0 {
                        mul *= dp[i - 1][j];
                    }

                    // update the answer or the {dp}
                    if i == t.len() - 1  && dp[i - 1][j] > 0 {
                        //println!("adding {}", mul);
                        ans += dp[i - 1][j];
                    } else if i < t.len() - 1 {
                        for k in j+1..s.len() {
                            dp[i][k] += mul;
                        }
                    }
                }
            }
        }
        //println!("{:?}", dp);
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

        assert_eq!(Solution::num_distinct(
            String::from("aabb"),
            String::from("abb")),
                   2);

        assert_eq!(Solution::num_distinct(
            String::from(""),
            String::from("a")),
                   0);

        assert_eq!(Solution::num_distinct(
            String::from("a"),
            String::from("b")),
                   0);

        assert_eq!(Solution::num_distinct(
            String::from("aaa"),
            String::from("a")),
                   3);
    }
}

fn main() {
    assert_eq!(Solution::num_distinct(
        String::from("abcdac"),
        String::from("ac")),
               3);
}
