use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Result;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Movement {
    num_steps: i32
}

fn main()  -> Result<()> {
    let a = Movement { num_steps: 3 };
    let mut file = File::create("move.bin")?;
    let serialized = serde_json::to_string(&a)?;
    file.write(serialized.as_bytes())?;

    // TODO: figure out how to read the contents of the file back in and deserialize that as a
    // Movement struct as the below is not doing that what you want
    let deserialized: Movement = serde_json::from_str(&serialized)?;

    



    Ok(())
    
}
