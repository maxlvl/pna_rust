use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Result;
use std::io::BufReader;
use std::io::Write;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Movement {
    num_steps: i32
}

fn main()  -> Result<()> {
    // create movement struct
    let a = Movement { num_steps: 3 };

    // create file
    let mut file = File::create("move.bin")?;

    // serialize struct and write to file
    let serialized = serde_json::to_string(&a)?;
    file.write(serialized.as_bytes())?;

    // reopen file, read from file and deserialize back into struct
    let open_file = File::open("move.bin")?;
    let mut buf_reader = BufReader::new(open_file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    let movement: Movement = serde_json::from_str(&contents)?;
    println!("{:?}", a);
    println!("{:?}", serialized);
    println!("{:?}", movement);

    Ok(())
    
}
