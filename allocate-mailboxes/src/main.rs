struct Solution {}
impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {

        println!("{:?}", houses);

        let mut left_cost : Vec<i32>
            = houses
            .windows(2)
            .enumerate()
            .scan(0, |cost, x| {
                println!("{:?}", x);
                let diff: i32 = x.1[1] - x.1[0];
                *cost += diff * (x.0 as i32 + 1);
                Some(*cost)
            })
            .collect();

        println!("{:?}", left_cost);

        let mut right_cost : Vec<i32>
            = houses
            .iter()
            .rev()
            .collect::<Vec<_>>()
            .windows(2)
            .enumerate()
            .scan(0, |cost, x| {
                println!("{:?}", x);
                let diff: i32 = x.1[0] - x.1[1];
                *cost += diff * (x.0 as i32 + 1);
                Some(*cost)
            })
            .collect();

        println!("{:?}", right_cost);



        1
    }
}

fn main() {
    Solution::min_distance(vec![1, 2, 5], 2);
}
