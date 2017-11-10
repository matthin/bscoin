extern crate byteorder;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate openssl;

use byteorder::{LittleEndian, WriteBytesExt};
use openssl::sha::sha256;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: [u8; 32],
    pub hash: [u8; 32],
    pub timestamp: i64,
    pub data: String,
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
