fn main() {
    /*
    for i in 0..=1usize {
        let f : usize = i;
    }
    let l : i32 = (2 as f32).log2().ceil() as i32;
    println!("{}", l);
     */
    // binary search - find the closest number to 0 in a given array.
    let array: Vec<i32> = vec![3, 2, 1, 0, -1, -2, -3];
    let mut slice = &array;
    let mut offset = 0;
    while slice.len() > 0 {
        if slice.len() == 1 {
            println!("{}", offset);
        } else {

        }
    }
}
