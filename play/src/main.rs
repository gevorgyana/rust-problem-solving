fn main() {

    /*
    for i in 0..=1usize {
        let f : usize = i;
    }
    let l : i32 = (2 as f32).log2().ceil() as i32;
    println!("{}", l);
     */

    let mut answer: usize = usize::max_value();
    // Binary search - find the closest number to 0 in a given array.
    let array: Vec<i32> = vec![3, 2, 1, 1, -1, -2, -2, -2, -3];
    let mut slice: &[i32] = &array;
    let mut offset: usize = 0;
    while slice.len() > 0 {
        if slice.len() == 1 {
            println!("{}", offset);
            answer = offset;
            break;
        } else if slice.len() == 2 {
            if slice[0].abs() < slice[1].abs() {
                answer = offset;
            } else {
                answer = offset + 1;
            }
            println!("{}", answer);
            break;
        }
        else {
            let middle_element: usize;
            if slice.len() % 2 == 0 {
                middle_element = slice.len() / 2 - 1;
            } else {
                middle_element = slice.len() / 2;
            }
            println!("checking array[{}]", middle_element + offset);
            println!("checking slice[{}]", middle_element);
            println!("slice before: {:?}", slice);
            if array[middle_element + offset] < 0 {
                slice = &slice[0..=middle_element];
                println!("slice after: {:?}", slice);
            } else if array[middle_element + offset] > 0 {
                slice = &slice[middle_element..slice.len()];
                offset += middle_element;
                println!("offset is {}", offset);
                println!("slice after: {:?}", slice);
            } else {
                println!("{}", middle_element + offset);
                answer = middle_element + offset;
                break;
            }
        }
    }
}
