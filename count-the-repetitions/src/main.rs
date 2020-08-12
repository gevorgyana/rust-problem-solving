struct Solution {}

impl Solution {
    pub fn get_max_repetitions(a: String, n1: i32, b: String, n2: i32) -> i32 {
        let mut vis_start: std::collections::HashSet::<usize>
            = std::collections::HashSet::<usize>::new();

        {
            // find what prefix of b is contained in a
            let mut prefix = 0;
            let mut start: usize = 0;

            while prefix < b.len() {
                let current_prefix = prefix;
                println!("looking for {:?}",
                         b.chars().nth(current_prefix));
                for i in start..a.len() {
                    if a.chars().nth(i) == b.chars().nth(prefix) {
                        prefix += 1;
                        start = i + 1;
                    }
                }

                if current_prefix == prefix {
                    if prefix == 0 {
                        println!("have nout found anything at all");
                    }
                    println!("have not found anyting past {}",
                             prefix - 1);
                    break;
                }
            }

            println!("success with start {}", start);
            if start == a.len() || vis_start.contains(&start) {
                println!("found a cycle");
            } else {
                vis_start.insert(start);
            }

        }

        1
    }
}

fn main() {
    Solution::get_max_repetitions
        (String::from("a"), 1, String::from("a"), 1);
}
