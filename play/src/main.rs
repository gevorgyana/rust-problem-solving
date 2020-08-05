struct S {
    field: i32,
}

fn main() {
    let s = S {field: 0};
    let mut v: Vec<S> = vec![];
    for i in 0..10 {
        if i == 0 {
            v.push(s);
        }
        /* this does not work either */
        match i {
            0 => { v.push(s); },
            _ => (),
        }
    }
}
