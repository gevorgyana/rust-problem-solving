fn main() {
    for i in 0..1 {
        let mut set: std::collections::HashSet<(i32, i32)> =
            std::collections::HashSet::new();
        set.insert((1, 1));
        for j in 0..1 {
            set.insert((1, 1));
        }
    }
}
