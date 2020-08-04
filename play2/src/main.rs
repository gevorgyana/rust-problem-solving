fn main() {
    foo();
}

fn foo() {
    // slices.
    // let vec: Vec<i32> = (1..=1).collect();
    // ranges.
    // let range: std::ops::RangeInclusive<i32> = (1..=1);
    // println!("{}", range.count());
    let fake_container_all_floors: Vec<i32> = (1..=5).collect();
    let other_fake_container: &[i32] = &fake_container_all_floors[0..5];
    println!("{:?}", other_fake_container);
}
