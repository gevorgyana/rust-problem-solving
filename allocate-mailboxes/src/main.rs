struct Solution {}
impl Solution {

    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {

        println!("{:?}", houses);
        let prefix_by_house
            = Self::min_distance_base(&houses);
        println!("{:?}", prefix_by_house);

        // As usual, {i} is for # of houses from the left that are used
        // {j} is the amount of resource that we have - in this case
        // houses
        let mut dp: Vec<Vec<i32>> = vec![];
        for i in 0..=k {
            dp.push(
                [0].repeat(houses.len() + 1)
            );
        }



        1
    }

    pub fn min_distance_base(houses: &Vec<i32>) ->
        Vec<i32>
    {
        // at the first house, we have 0 houses to the left and 0 cost of
        // reaching them from this point.
        std::iter::once(0)
            .chain(
                houses
                    .iter()
                    .collect::<Vec<&i32>>()
                    .windows(2)
                    .enumerate()
                // we have started from this value and will go on with
                // it here in the loop
                    .scan(0,
                          |
                          cost: &mut i32,
                          x: (usize, &[&i32])
                          | {
                              let diff: i32 = x.1[1] - x.1[0];
                              *cost += diff * (x.0 as i32 + 1);
                              Some(*cost)
                          }
                    )
            )
            .collect()
    }
}

fn main() {
    Solution::min_distance(vec![1, 3, 5], 2);
}
