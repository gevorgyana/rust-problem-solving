struct Solution {}
impl Solution {

    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {

        println!("{:?}", houses);
        let prefix_by_house
            = Self::min_distance_base(&houses);
        println!("{:?}", prefix_by_house);

        1
    }

    /// arrange one mailbox among N houses.
    fn basic_arrangement(prefix_by_house: &[i32],
                         houses: &Vec<i32>,
                         i: usize) {

        // linear search
        for i in 0..prefix_by_house.len() - 1 {
            for j in houses[i]..houses[i + 1] {
                println!("{}", j);
            }
        }
    }

    fn min_distance_base(houses: &Vec<i32>) ->
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
