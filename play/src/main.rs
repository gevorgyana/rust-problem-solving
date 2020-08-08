fn main() {
    let x = vec![1, 2, 3];
    let mut y = &x[0..3];
    println!("{:?}", y);
    y = &y[1..3];
    println!("{:?}", y);
    println!("{:?}", y[0]);
}
