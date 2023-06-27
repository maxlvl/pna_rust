use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::result::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Movement {
    num_steps: i32,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("move.bin").unwrap();

    for _ in 0..1000 {
        let m = Movement {
            num_steps: rand::thread_rng().gen(),
        };

        let serialized_movement = bson::ser::to_vec(&m).unwrap();
        file.write(&serialized_movement)?;
    }

    let open_file = File::open("move.bin")?;
    let mut buf_reader = BufReader::new(open_file);
    // read the whole file into a vector and deserialize it into separate structs
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    let mut cursor = Cursor::new(contents);
    let mut deserialized_contents = Vec::<Movement>::new();

    for _ in 0..1000 {
        let mut bytes_length = [0u8; 4];
        cursor.read_exact(&mut bytes_length)?;
        // gets the length of the bytes to read
        let length = i32::from_le_bytes(bytes_length) as usize;

        let mut document_bytes = vec![0u8; length];
        cursor.set_position(cursor.position() - 4);
        cursor.read_exact(&mut document_bytes)?;

        let deser_movement: Movement = bson::de::from_slice(&document_bytes)?;
        println!("{:?}", &deser_movement);
        deserialized_contents.push(deser_movement);
    }

    Ok(())
}
