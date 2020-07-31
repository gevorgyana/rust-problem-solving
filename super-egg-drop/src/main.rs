struct Solution {}
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        println!("{} {}", k, n);
        if n == 0 {
            return 0;
        }

        if k == 1 {
            return n;
        }

        // todo log instead
        if k > n {
            panic!("does not happen");
        }

        if n % 2 == 0 {
            1 + std::cmp::max(
                Self::super_egg_drop(k - 1, n / 2 - 1),
                Self::super_egg_drop(k, n / 2)
            )
        } else {
            1 + Self::super_egg_drop(k - 1, n / 2)
        }
    }
}

fn main() {
    assert_eq!(
        Solution::super_egg_drop(1, 2),
        2
    );
    println!("---");
    assert_eq!(
        Solution::super_egg_drop(2, 6),
        3
    );
    println!("---");
    assert_eq!(
        Solution::super_egg_drop(3, 14),
        4
    );
    println!("---");
}
