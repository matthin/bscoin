extern crate byteorder;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate openssl;

use byteorder::{LittleEndian, WriteBytesExt};
use openssl::sha::sha256;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: [u8; 32],
    pub hash: [u8; 32],
    pub timestamp: i64,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub list: Vec<Block>,
}

#[derive(Debug)]
pub enum BsError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for BsError {
    fn from(err: io::Error) -> BsError {
        BsError::Io(err)
    }
}

impl From<serde_json::Error> for BsError {
    fn from(err: serde_json::Error) -> BsError {
        BsError::Json(err)
    }
}

impl Block {
    pub fn calculate_hash(&mut self) -> io::Result<()> {
        let mut buffer = Vec::new();
        buffer.write_u64::<LittleEndian>(self.index)?;
        buffer.write_i64::<LittleEndian>(self.timestamp)?;
        for c in self.data.bytes() {
            buffer.write_u8(c)?;
        }

        self.hash = sha256(&buffer);

        Ok(())
    }
}

impl Blockchain {
    pub fn load() -> Result<Blockchain, BsError> {
        let mut f = File::open("blocks.json")?;
        let mut buffer = String::new();
        let _ = f.read_to_string(&mut buffer);
        let b: Blockchain = serde_json::from_str(&buffer)?;
        Ok(b)
    }

    pub fn save(&self) -> Result<(), BsError> {
        let mut f = File::create("blocks.json")?;
        let s = serde_json::to_vec(self)?;
        f.write_all(&s)?;
        Ok(())
    }

    pub fn new() -> Blockchain {
        Blockchain { list: Vec::new() }
    }

    pub fn next_block(&mut self, data: String) -> io::Result<Block> {
        let prev = self.list.last().unwrap();

        let mut block = Block {
            index: prev.index + 1,
            previous_hash: prev.hash,
            hash: [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            timestamp: time::now().to_timespec().sec,
            data: data,
        };
        block.calculate_hash()?;

        Ok(block)
    }
}
