struct Solution {}
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        // As usual, create (N + 1) x (M + 1) values for easier indexing
        // and holding the results of invalid queries.
        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=nums.len() {
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
                dp.push([0].repeat(m as usize));
            }
        }

        // first row
        for i in 0..nums.len() {

        }

        1
    }
}

fn main() {
    Solution::split_array(
        vec![1, 2, 3], 1
    );
}
