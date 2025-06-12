use crate::io::Cursor;
use binrw::io::*;
use binrw::BinReaderExt;
use binrw_tahoe::lib::*;
use std::fs::File;
use std::io;
use std::result;
// this works for files small enough to fit into v1 ? We hope?

fn main() -> result::Result<(), io::Error> {
    let _ = read_cap("1of2.0");
    let mut part2 = File::open("1of2.1")?;
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part2.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(pile_of_bytes);
    let share: Share = dbg!(rdr.read_be().unwrap());

    let ueb = str::from_utf8(share.uri_ext);
    dbg!(ueb);
    Ok(())
}

fn read_cap(filename: &str) -> result::Result<Share, io::Error> {
    let mut part1 = File::open(filename)?;
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part1.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(pile_of_bytes);
    let share: Share = rdr.read_be().unwrap();
    Ok(share)
}

#[cfg(test)]
mod tests {
    // for 1of2.0 and 1of2.1 :
    // wellKnownConvergenceSecret = decodeBase32Unpadded "lcngfrvgaksfwrelc6ae5kucb3zufssoe6cj74rozcqibnl6uy2a"
    // cap = "URI:CHK:pyv3qypbpk6knq5ozeibenuubq:jh3twlgmxtytwqtzn6jtbsfy2w574ybkcnalurlnlq2snuu3j5da:1:2:56"
    use super::*;
    #[test]
    fn it_works() {
	let s = read_cap("1of2.0").unwrap();
	assert_eq!(s.lease_version, 2);
    }
}
