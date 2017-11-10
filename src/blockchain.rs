extern crate serde;
extern crate serde_json;
extern crate time;

use block::Block;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::File;
use error::BsError;

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub list: Vec<Block>,
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
