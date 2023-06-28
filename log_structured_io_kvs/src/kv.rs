#[allow(dead_code)]
use std::path::Path;
pub struct KvStore {
    stuff: i32,
}

impl KvStore {
    pub fn open(path: &Path) -> std::io::Result<()> {
        panic!()
    }

    pub fn set(key: String, val: String) -> std::io::Result<()> {
        panic!()
    }

    pub fn get(key: String) -> std::io::Result<()> {
        panic!()
    }

    pub fn remove(key: String) -> std::io::Result<()> {
        panic!()
    }
}
