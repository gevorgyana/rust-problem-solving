// how to use mut refs in enumerate()

fn main() {
    let mut v: Vec<Vec<i32>> = vec![vec![1, 2, 3]];
    for (a, mut b) in v.iter_mut().enumerate() {
        // this is not possible, so in current problem instead I am
        // extending the existing vector instead of creating a new one
        // b = &mut vec![1];
        b.reserve(10);
    }
}
