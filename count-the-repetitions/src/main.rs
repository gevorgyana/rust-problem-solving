fn main() {
    assert_eq!(
        Solution::get_max_repetitions
        // aaa aaa aaa aaa
        //  1  2 3  4  5 6
            (String::from("aaa"), 4, String::from("aa"), 1),
        6 // can get 6 "aa"s, / 2 as each block has 2 of them
    );
}
struct Solution {}
impl Solution {
    pub fn get_max_repetitions(ac: String, n1: i32, b: String, n2: i32) -> i32 {

        println!("a {:?}", ac);
        println!("b {:?}", b);
        println!("n1 {}", n1);
        println!("n2 {}", n2);

        if n1 == 0 || n2 == 0 {
            return 0;
        }

        let mut cache: std::collections::HashSet::<usize>
            = std::collections::HashSet::<usize>::new();
        let mut a = ac.clone();
        let mut a_position = 0;
        let mut a_used_counter = 0;
        let mut b_iter = 0;
        let mut trail: Vec<(usize, usize)>
            = vec![];
        let mut trail_positions: Vec<usize>
            = vec![];
        loop {

            // println!("loop");

            let mut prefix_recognized = 0;
            while prefix_recognized < b.len() {

                // println!("while");

                let prefix_recognized_cur = prefix_recognized;
                for i in a_position..a.len() {
                    if a.chars().nth(i)
                        == b.chars().nth(prefix_recognized)
                    {
                        a_position = i + 1;
                        /*
                        println!("!found b[{}] at a[{}]",
                                 prefix_recognized,
                                 i
                        );
                         */
                        prefix_recognized += 1;
                    }
                }

                if prefix_recognized == prefix_recognized_cur
                    || (prefix_recognized < b.len() &&
                        a_position == a.len())
                {
                    if a.contains(b.chars().nth(prefix_recognized).unwrap()) {
                        // println!("!one more chance");
                        a_position = 0;
                        a_used_counter += 1;
                        a = ac.clone();
                    } else {
                        panic!("stop - failure");
                    }
                }
            }

            /*
             * we can even go further and remember the relations
             * a[i] == b[j] and cache them.
             */

            // println!("?success with cached value {}", a_position);

            if cache.contains(&a_position) {
                trail.push((b_iter + 1, a_used_counter + 1));
                trail_positions.push(a_position);
                break;
            } else {
                // println!("found 1 more at {}", a_position);
                b_iter += 1;
                cache.insert(a_position);
                trail.push((b_iter, a_used_counter + 1));
                trail_positions.push(a_position);
            }
        }

        println!("the trail {:?}", trail);
        println!("trail positions {:?}", trail_positions);

        let mut as_spent: usize = 0;
        let mut bs_acquired: usize = 0;

        let mut first_cyclic: usize = 0;
        let n = trail.len();
        for i in 1..n {
            if trail_positions[n - 1 - i] == trail_positions[n - 1] {
                first_cyclic = n - i;
            }
        }
        println!("first cyclic {}", first_cyclic);

        // before the cycle we try to get to the cycle
        for i in 0..first_cyclic {
            if trail[i].1 <= n1 as usize {
                bs_acquired = trail[i].0;
                as_spent = trail[i].1;
            }
        }

        println!("spent {}, acquired {}",
                 as_spent,
                 bs_acquired
        );

        if (as_spent == n1 as usize) {
            return bs_acquired as i32;
        }

        // fast-forward
        let as_avail_when_cycling
            = n1 as usize - as_spent;

        println!("as_avail_when_cycling {}", as_avail_when_cycling);

        let cycle_gain
            = trail[n - 1].0 - trail[first_cyclic - 1].0;
        let cycle_cost
            = trail[n - 1].1 - trail[first_cyclic - 1].1;

        println!("cycle-gain {}", cycle_gain);
        println!("cycle-cost {}", cycle_cost);


        let cycles_done
            = as_avail_when_cycling
            / cycle_cost
            ;

        println!("cycles done {}",
                 cycles_done
        );

        bs_acquired
            += cycles_done
            * cycle_gain;

        as_spent
            += cycles_done
            * cycle_cost
            ;

        println!("Updated data: spent {}, acquired {}",
                 as_spent,
                 bs_acquired
        );

        let as_avail_after_cycle = n1 as usize - as_spent;

        println!("after all we have this many resources {}",
                 as_avail_after_cycle
        );

        let mut best_residue: usize = 0;
        for i in first_cyclic..n - 1 {

            println!("trying this i-th residue {}", i);
            println!("its cost is {}",
                     (trail[i].1 - trail[first_cyclic - 1].1)
            );

            if as_avail_after_cycle
                == (trail[i].1 - trail[first_cyclic - 1].1)
            {
                best_residue = std::cmp::max(
                    best_residue,
                    (trail[i].0 - trail[first_cyclic - 1].0)
                );

                println!("best residue {}",
                         best_residue
                );
            }
        }

        as_spent += as_avail_after_cycle;
        bs_acquired
            += best_residue;

        bs_acquired as i32 / n2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cs1() {
        assert_eq!(
            Solution::get_max_repetitions
            // have this: a
            // need blocks of type: a
            // can have exactly one of those
                (String::from("a"), 1, String::from("a"), 1),
            1
        );
    }

    #[test]
    fn cs2() {
        assert_eq!(
            // have this: aaa
            // need blocks of type: aa
            // can have 1
            Solution::get_max_repetitions
                (String::from("aaa"), 1, String::from("aa"), 1),
            1
        );
    }

    #[test]
    fn cs3() {
        assert_eq!(
            // have this: aaa aaa aaa
            //             1  2 3  1
            // need these: aa
            Solution::get_max_repetitions
                (String::from("aaa"), 3, String::from("aa"), 1),
            4
        );
    }

    #[test]
    fn cs4() {
        assert_eq!(
            Solution::get_max_repetitions
            // aaa aaa aaa aaa
            //  1  2 3  1  2 3
                (String::from("aaa"), 4, String::from("aa"), 1),
            6 // can get 6 "aa"s, / 2 as each block has 2 of them
        );
    }

    #[test]
    fn cs5() {
        assert_eq!(
            Solution::get_max_repetitions
                (String::from("a"), 1, String::from("a"), 2),
            0
        );
    }

    fn lc1() {
        assert_eq!(
            // have this
            // acb acb acb acb acb
            //   1   1   1   1
            // need these groups
            // ab ab
            Solution::get_max_repetitions
                (String::from("acb"), 4, String::from("ab"), 2),

            2
        );
    }

    #[test]
    fn lc2() {

        /*
         * x - offset 0
         * if we remember the offset (where we say this value), then
         * we can do it.
         */

        println!("!baba");
        println!("!baab");
        assert_eq!(
            // have this: baba baba baba baba baba ..
            //                 x      x       x
            // (11 times)
            // need blocks of type: baab
            // we can get 2 baab's from every 3 baba's
            // 11 / 3 = 3, so we can get 3 * 2 = 6 easily
            // then we are left with 2 as a residue, we will get 1 from
            // it, which is 7 in total
            Solution::get_max_repetitions
                (String::from("baba"), 11, String::from("baab"), 1),
            7
        );
    }

    #[test]
    fn lc3() {
        assert_eq!(
            // bacaba bacaba bacaba
            //            x      x
            // here, we have the cycle that starts not from the first
            // position, so we'll have a problem; currently this does
            // not pass
            Solution::get_max_repetitions
                (String::from("bacaba"), 3, String::from("abacab"), 1),
            2
        );
    }
}
