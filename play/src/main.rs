// how to use mut refs in enumerate()

fn main() {
    let mut v: Vec<Vec<i32>> = vec![vec![1, 2, 3]];
    // move is happening here, the value is moved from the vector
    // doing do will yield an error
    // for (a, &mut b) in v.iter_mut().enumerate() {
    // (cannot move borrow mut twice, so we have to move)
    for (a, mut b) in v.iter_mut().enumerate() {
        b.reserve(10);
    }
}
