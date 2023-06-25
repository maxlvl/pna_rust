use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Movement {
    num_steps: i32
}

fn main() {
    let movement: Movement = Movement { num_steps: 3 };
    let buff: Vec<u8> = ron::ser::to_string(&movement).unwrap().into();

    println!("{:?}", buff);
}
