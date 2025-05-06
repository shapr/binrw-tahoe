use binrw::io::*;
use binrw::*;
use std::fs::File;
use std::io;
use std::result;
// this works for files small enough to fit into v1 ? We hope?
#[derive(BinRead, PartialEq, PartialOrd, Debug)]
struct Share {
    lease_version: u32,
    lease_data_length: u32,
    lease_count: u32,
    share_version: u32,
    block_size: u32,
    data_size: u32,
    data_offset: u32,
    plaintxt_hash_tree_offset: u32,
    cryptxt_hash_tree_offset: u32,
    block_hashes_offset: u32,
    share_hashes_offset: u32,
    uri_ext_offset: u32,
    // data starts now!
    // #[br(big, count = data_size)]
    // share_data: Vec<u8>,
}
fn main() -> result::Result<(), io::Error> {
    {
        let mut part1 = File::open("1of2.0")?;
        let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
        part1.read(&mut pile_of_bytes).unwrap();
        let mut rdr = Cursor::new(pile_of_bytes);
        let _: Share = dbg!(rdr.read_be().unwrap());
    }
    let mut part2 = File::open("1of2.1")?;
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part2.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(pile_of_bytes);
    let _: Share = dbg!(rdr.read_be().unwrap());

    Ok(())
}
