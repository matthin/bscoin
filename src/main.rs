extern crate bscoin;

use bscoin::blockchain::Blockchain;

fn main() {
    let chain = Blockchain::load().unwrap();
    chain.save().unwrap();
}
