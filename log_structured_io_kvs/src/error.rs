use std::io;
#[derive(Debug)]
pub enum KvsError {
    Io(io::Error),
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}
pub type Result<T> = std::result::Result<T, KvsError>;
