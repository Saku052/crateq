use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
pub struct Wal {
    file: File,
}

pub enum Op {
    Insert { key: Vec<u8>, value: Vec<u8>},
    Delete { key:Vec<u8>},
}

impl Wal {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Wal {file})
    }

    pub fn append(&mut self, op: &Op) -> io::Result<()>;
    pub fn replay(path: &Path) -> io::Result<Vec<Op>>;
}