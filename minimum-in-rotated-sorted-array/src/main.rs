fn is_rotation_index(vec: &Vec<i32>, idx: usize) -> bool {
    // size N, id 0
    // vec[0] vs vec[N - 1]
    // size N, id 1
    // vec[1] vs vec[N - 2]
    // size N, id N / 2 - 1
    // vec[N / 2 - 1] vs vec[N - N / 2 + 1 = N / 2 + 1 or N / 2] -> okay
    // size N, id N / 2
    // vec[N / 2] vs vec[N - N / 2 = N / 2 or N / 2 - 1] -> forbidden
    assert_eq!(idx < vec.len() / 2, true);
    vec[idx] > vec[vec.len() - idx - 1]
}

fn solve(vec: &Vec<i32>) {
    // rust bin search with custom predicate; bin search on
    // the two equally distant elements from the center of array

}

fn main() {

    println!("Hello, world!");
}
