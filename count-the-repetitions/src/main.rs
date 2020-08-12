struct Solution {}

impl Solution {
    pub fn get_max_repetitions(a: String, n1: i32, b: String, n2: i32) -> i32 {

        let mut vis_start: std::collections::HashSet::<usize>
            = std::collections::HashSet::<usize>::new();
        let mut prefix: usize = 0;
        let mut start: usize = 0;
        let mut wip_a = a.clone();
        let mut hm_a_needed: usize = 0;

        // the goal now is to stop when we hit the same period
        while prefix < b.len() {
            let current_prefix = prefix;
            println!("looking for {:?}",
                     b.chars().nth(current_prefix));
            // trying to find it in the current range
            for i in start..wip_a.len() {
                if wip_a.chars().nth(i) == b.chars().nth(prefix) {
                    prefix += 1;
                    start = i + 1;
                }
            }

            if current_prefix == prefix {
                println!("add one more A or stop?");
                // it makes sense to add one more when we have it
                // before the {start} - otherwise we should quit
                let mut stop = true;
                for i in 0..start {
                    if a.chars().nth(i) == b.chars().nth(prefix) {
                        stop = false;
                        break;
                    }
                }
                if stop {
                    panic!("it is officially done");
                } else {
                    println!("it makes sense to add one more A");
                    wip_a.push_str(&a.clone());
                    hm_a_needed += 1;
                    continue;
                }
            }

            println!("success with start {}", start - 1);
            if start == a.len() || vis_start.contains(&start) {
                println!("found a cycle");
            } else {
                vis_start.insert(start);
            }
        }

        println!("done with this many concatenations {}", hm_a_needed);

        /*
         * We know that we can fit BB..B (K1 reps) into AA..A (N1 reps),
         * and that from that point on, we will keep repeating the
         * pattern. We need to solve for N1 = N and K1 = K.
         * We have the following map:
         * hm_bs -> (hm_a_needed, completed_at)
         * and we also know the length of the period.
         * so we fast-forward to the case of K1 = K by dividing it
         * by the lenght of the period, and do not forget to multiply
         * the # of A's used by that same value. See that
         * K_residual -> (hm_a_needed, pos) - we know that we need this
         * {hm_a_needed} As to get the thing done. Do we have this many?
         * Yes -> done.

         * The problem is that we can only do this trick with the small
         * strings - each 100 chars at max.

         * small upadte: what if there is no sense in binary search at
         * all? just convert the string [s2; n2] in a naive way and
         * waste 10^6 of time for that, then do the following: try
         * the algorithm with this string and arbitrary # of A's.
         * We have a string B that is of length 10^8.
         * We have the period whoose length is 100 at max. So we spend
         * 100 units of memory and 10^8 time to finish. We then can
         * devise the number M and return it. We know we have this map
         * of length 100, and we fast-forward as explained in previous
         * paragraph. This should work.
         */

        1
    }
}

/*
 * one more update:
 * here is what we need:
 * keep trying to find B in A, and find as many of B's in A as you can
 * - until you hit the visited cache, where the key is the position
 * after a full match. As a result, we know that 1 B needs Y1 A's,
 * 2 B's need Y2 A's, ..., X1 B's need Y3 A's. So we know the whole
 * pattern. We can now fast-forward by to the point where we know
 * how many A's we need to get n2 B's. How do we fast-forward?
 * Take the n2, divide it into the maximum key from the map that we
 * know - x - we get exactly this number of B's for this - mem(x) -
 * number of A's in a row. Then calculate {n1 - mem(x)} - we need to
 * get this # of A's, search the whole map to find the closet we can
 * get to it. Okay, so we know exactly how many A's we need to cover
 * the whole [B; n2] string. We need to know how many of A's have left,
 * and spend them on B's now. So we know the value of the function V,
 * and we are going to look for the argument that satifies it in the
 * best way (max x : mem(x) <= V, idelly = V).
 *
 */
fn main() {
    Solution::get_max_repetitions
        (String::from("a"), 1, String::from("a"), 1);
}
