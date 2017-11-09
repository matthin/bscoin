extern crate bscoin;

fn main() {
    let chain = bscoin::block::Blockchain::load().unwrap();
    chain.save().unwrap();
}
