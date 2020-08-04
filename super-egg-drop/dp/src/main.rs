struct Solution {}
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {

        // Need this fake array to work with slices, as slices have
        // to point at real memory. This is done to use slices in
        // binary search, as it makes it less error-prone. The alternative
        // is to use regular integers like in C++.
        let fake_container_all_floors: Vec<usize>
            = (1..=n as usize).collect();

        // If we have enough steps to perform full binary search, of
        // course we can do it. But take logarith of N + 1, as even if
        // we have 1 floor as problem size, we still need to check it.
        let log2_n: i32 = ((n + 1) as f32).log2().ceil() as i32;
        if k >= log2_n {
            return log2_n;
        }

        // Huge optimization by itself - we will need at most log(n)
        // values, and maybe even less, if we do not have enough {k}.
        let k_bound = std::cmp::min(k, log2_n);

        // dp[N][K]
        let mut dp : Vec<Vec<i32>> = vec![];
        for _i in 0..=n as usize {
            dp.push([0].repeat((k_bound + 1) as usize));
        }

        // When we have only one egg, it doesn't make sense to do
        // anything other than gradually trying each floor, starting from
        // the lowest one and moving up one floor for as long as the egg
        // does not crack.
        for i in 1..=n as usize {
            dp[i as usize][1] = i as i32;
        }

        // Need this as induction base.
        for i in 1..=k_bound as usize {
            dp[1][i as usize] = 1;
        }

        println!("{:?}", dp);

        for i in 2..=n as usize {

            println!("calculating for i = {}", i);

            // Current complexity: O(N * log2(N) * log2(N)), which is
            // almost as good as the version without dp.
            // The bottleneck is having to calculate all the values of
            // the amount of stores that we have - that is the price we
            // have to pay to say that we cover every solution. We do not
            // have such problems with top-down approaches.
            //
            // -- N for each size row in the matrix that represents the
            // amount of floors for a subproblem.
            // -- log2(N) is for how many eggs we need to calculate to
            // until we reach full potential, where we might execute
            // complete binary search.
            // -- log2(N) is for how many values we need to check to
            // calculate the answer for a given amout of floors and eggs.
            for k in 2..=k_bound as usize {

                println!("calculating for k = {}", k);

                // Further optimization! May actually remove this... It
                // does not help with complexity.
                /*
                let log2_i : usize = ((i + 1) as f32).log2().ceil()
                    as usize;
                if k > log2_i {
                    dp[i][k] = dp[i][log2_i];
                    continue;
                }
                 */

                let mut answer: usize = usize::max_value();
                let mut slice: &[usize]
                    = &fake_container_all_floors[0..i];
                let mut offset: usize = 0;

                println!("started binary search with slice {:?}",
                         slice
                );

                // Finds the index that is closest to equalising the
                // costs of solving two subtasks. In other words, find
                // such {i} that cost(upper(i), lower(i)) is closest to 0
                while slice.len() > 0 {
                    if slice.len() == 1 {
                        println!("this index won {}", offset);
                        answer = fake_container_all_floors[offset];
                        break;
                    } else if slice.len() == 2 {

                        println!("trapped with 2 elements {:?}", slice);

                        let current_floor_left
                            = fake_container_all_floors
                            [offset];

                        let cracks_left: i32
                            = dp[current_floor_left - 1][k - 1];

                        let survives_left: i32
                            = dp[i - current_floor_left][k];

                        let discriminant_left: i32 = cracks_left
                            - survives_left;

                        let current_floor_right
                            = fake_container_all_floors
                            [offset + 1];

                        let cracks_right: i32
                            = dp[current_floor_right - 1][k - 1];

                        let survives_right: i32
                            = dp[i - current_floor_right][k];

                        let discriminant_right: i32 = cracks_right
                            - survives_right;

                        if discriminant_left.abs()
                            < discriminant_right.abs()
                        {
                            answer
                                = fake_container_all_floors[offset];
                        } else {
                            answer
                                = fake_container_all_floors[offset + 1];
                        }

                        println!("this floor won {}", answer);
                        break;
                    }
                    else {
                        let middle_element: usize;
                        if slice.len() % 2 == 0 {
                            middle_element = slice.len() / 2 - 1;
                        } else {
                            middle_element = slice.len() / 2;
                        }

                        // Could use the idea that the index in the array
                        // points at a value {i + 1}, but let's use the
                        // slice the straightforward way.

                        let current_floor = fake_container_all_floors
                            [middle_element + offset];
                        println!("dropping from current floor : {}",
                                 current_floor
                        );

                        let cracks: i32 = dp[current_floor - 1][k - 1];
                        println!("cracks with cost {}", cracks);
                        println!("cost from dp[{}][{}]",
                                 current_floor - 1, k - 1);
                        let survives: i32 = dp[i - current_floor][k];
                        println!("survives with cost {}", survives);
                        println!("cost from dp[{}][{}]",
                                 i - current_floor, k);

                        // These function values are sorted in decreasing
                        // order for this sequence of floors: 1, 2, .., i.
                        let discriminant = cracks - survives;
                        if discriminant > 0 {
                            slice = &slice[0..=middle_element];
                            println!("slice after: {:?}", slice);
                        } else if discriminant < 0 {
                            slice = &slice[middle_element..slice.len()];
                            offset += middle_element;
                            println!("offset is {}", offset);
                            println!("slice after: {:?}", slice);
                        } else {
                            println!("this floor won {}",
                                     fake_container_all_floors
                                     [middle_element + offset]
                                     );
                            answer = fake_container_all_floors
                                [middle_element + offset];
                            break;
                        }
                    }
                }

                let winner_floor = answer;
                println!("final check: this floor won {}", winner_floor);
                let cracks: i32 = dp[winner_floor - 1][k - 1];
                let survives: i32 = dp[i - winner_floor][k];
                println!("final check: cracks with cost {}", cracks);
                println!("final check: survives with cost {}", survives);

                println!("setting dp[i][k] = {}",
                         std::cmp::max(cracks, survives) as i32 + 1);
                dp[i][k] = std::cmp::max(cracks, survives) as i32 + 1;
            }
        }
        println!("{:?}", dp);
        dp[n as usize][k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lc1() {
        assert_eq!(
            Solution::super_egg_drop(2, 9),
            4
        );
    }

    #[test]
    fn lc2() {
        assert_eq!(
            Solution::super_egg_drop(1, 2),
            2
        );
    }
}

fn main() {

    assert_eq!(
        Solution::super_egg_drop(2, 9),
        4
    );

    /*
    assert_eq!(
        Solution::super_egg_drop(4, 2000),
        16
    );
     */
    /*
    assert_eq!(
        Solution::super_egg_drop(4, 10000),
        16
    );
     */
}
