// read 2d vector from file into Vec
extern crate serde;

fn main() {
    let t: String
        = std::fs::read_to_string("test").unwrap().parse().unwrap();
    let v: Result<Vec<Vec<i32>>, _> = serde_json::from_str(
        &t
    );
    match v {
        Ok(inner) => {
            println!("Done {:?}", inner);
        },
        _ => (),
    }
}
