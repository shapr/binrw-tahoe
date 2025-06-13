use crate::io::Cursor;
use binrw::io::*;
use binrw::BinReaderExt;
use binrw_tahoe::lib::*;
use bitcoin_hashes::{sha256d, HashEngine};
use data_encoding::BASE32_NOPAD;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::result;
// this works for files small enough to fit into v1 ? We hope?

static URI_TAG: &str = "allmydata_uri_extension_v1";
static UEB_TAG: &str = "26:allmydata_uri_extension_v1,";

fn main() -> result::Result<(), io::Error> {
    let _ = read_cap("1of2.0");
    let mut part2 = File::open("1of2.1")?;
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part2.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(pile_of_bytes);
    let s: Share = dbg!(rdr.read_be().unwrap());
    let ueb_bytes = s.uri_ext;
    let the_ueb = read_ueb(&ueb_bytes);
    print!("{:?}", the_ueb);
    let taggy = tagged_hash(URI_TAG.as_bytes(), &ueb_bytes, 32);
    print!("{}", b2a(taggy));
    Ok(())
}
/*
sha256d value of the tagged hash
the tagged hash is the netstring of the tag, and the value
the netstring is the ascii value of the length of the tag: actual tag,
26:

sha256d digest of UEB_TAG + UEB bytes value, should be equal to:
jh3twlgmxtytwqtzn6jtbsfy2w574ybkcnalurlnlq2snuu3j5da from the capability string:
cap = "URI:CHK:pyv3qypbpk6knq5ozeibenuubq:jh3twlgmxtytwqtzn6jtbsfy2w574ybkcnalurlnlq2snuu3j5da:1:2:56"

*/

// pulled from "lafs"
pub fn tagged_hash(tag: &[u8], val: &[u8], truncate_to: usize) -> Vec<u8> {
    if truncate_to > 32 {
	panic!("truncate_to must be <= 32");
    }
    let mut engine = sha256d::Hash::engine();
    engine.input(&netstring(tag));
    engine.input(val);
    sha256d::Hash::from_engine(engine).to_byte_array()[0..truncate_to].to_vec()
}

// pulled from "lafs"
pub fn netstring(s: &[u8]) -> Vec<u8> {
    format!("{}:{},", s.len(), std::str::from_utf8(s).unwrap()).into_bytes()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Base32 {
    base32: String,
    bytes: Vec<u8>,
}

pub fn b2a(bytes: Vec<u8>) -> Base32 {
    Base32 {
	base32: BASE32_NOPAD.encode(&bytes).to_lowercase(),
	bytes,
    }
}

// impl TryFrom<String> for Base32 {
//     type Error = ();

//     fn try_from(base32: String) -> Result<Self, Self::Error> {
//	Base32::try_from(&base32 as &str)
//     }
// }

impl Display for Base32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), std::fmt::Error> {
	write!(f, "{}", self.base32)
    }
}

fn read_ueb(b: &[u8]) -> result::Result<UEB, io::Error> {
    let mut rdr = Cursor::new(b);
    let ueb: UEB = rdr.read_be().unwrap();
    Ok(ueb)
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
	assert_eq!(s.uri_ext_offset, 1600);
	assert_eq!(s.uri_ext_size, 302);
    }
}
