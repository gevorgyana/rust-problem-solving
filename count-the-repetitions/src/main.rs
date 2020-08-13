fn main() {

}
struct Solution {}
impl Solution {
    pub fn get_max_repetitions(ac: String, n1: i32, b: String, n2: i32) -> i32 {
        let mut cache: std::collections::HashSet::<usize>
            = std::collections::HashSet::<usize>::new();
        let mut a = ac.clone();
        let mut a_position = 0;
        let mut a_used_counter = 0;
        let mut b_iter = 0;
        let mut results: std::collections::HashMap::<usize, usize>
            = std::collections::HashMap::<usize, usize>::new();
        loop {
            let mut prefix_recognized = 0;
            while prefix_recognized < b.len() {
                let prefix_recognized_cur = prefix_recognized;
                for i in a_position..a.len() {
                    if a.chars().nth(i)
                        == b.chars().nth(prefix_recognized)
                    {
                        prefix_recognized += 1;
                        a_position += 1;
                    }
                }

                if prefix_recognized == prefix_recognized_cur
                    || (prefix_recognized < b.len() &&
                        a_position == a.len())
                {
                    if a.contains(b.chars().nth(prefix_recognized).unwrap()) {
                        a_position = 0;
                        a_used_counter += 1;
                        a = ac.clone();
                    } else {
                        panic!("stop - failure");
                    }
                }
            }

            // println!("success with cached value {}", a_position);

            if cache.contains(&a_position) {
                break;
            } else {
                b_iter += 1;
                cache.insert(a_position);
                results.insert(b_iter, a_used_counter + 1);
            }
        }

        // println!("results {:?}", results);

        let greater_key: (&usize, &usize)
            = results.iter().max().unwrap();
        // println!("greater key {:?}", greater_key);
        let hm_bs_can_get: usize
            = n1 as usize
            / *greater_key.1
            * *greater_key.0
            ;
        // println!("hm_bs w/o remainder {}", hm_bs_can_get);
        let hm_as_are_wasted: usize
            = hm_bs_can_get
            / *greater_key.0
            * *greater_key.1
            ;
        // println!("hm_as_ are wasted {}", hm_as_are_wasted);
        let remainder_of_as
            = n1 as usize - hm_as_are_wasted;
        // println!("remainder of as {}", remainder_of_as);
        let mut remainder_bs: usize
            = 0;
        if remainder_of_as != 0 {
            for i in results {
                if i.0 == remainder_of_as as usize {
                    remainder_bs = i.0;
                }
            }
        }
        // println!("hm_bs_from_remainder {:?}", remainder_bs);

        /*
         * If we insert consecutive elements into the map, as we are
         * going up to 100 positions, and on stopping at each position
         * we increase the # of b's we have found in some # of a's.
         * therefore, we are guaranteed that we will have a range of
         * the following form in the map: 0, 1, ..., N, where N < 100.
         */

        let res_hm_bs
            = remainder_bs
            + hm_bs_can_get
            ;

        // println!("can get this many bs {}", res_hm_bs);

        res_hm_bs as i32 / n2
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
}
