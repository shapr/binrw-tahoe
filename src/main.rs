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
use rs_merkle::{Hasher,MerkleTree};
use rs_merkle::algorithms::*;

use binrw_tahoe::lib::*;

// this works for files small enough to fit into v1 ? We hope?

static URI_TAG: &str = "allmydata_uri_extension_v1";
static UEB_TAG: &str = "26:allmydata_uri_extension_v1,";
//static CRYPT_SEG: &str = "allmydata_crypttext_segment_v1";
//static CRYPT_SEG: &str = "allmydata_plaintext_segment_v1";


struct CryptTextHashTree <'a> {
    //pub raw_data: Vec<u8>,
    pub raw_data: &'a[u8],
}


impl <'a> CryptTextHashTree<'a> {
    // XXX would be cool if we could return &[u8; 32] since we know it's 32 bytes
    pub fn node_hash(&self, node_number: usize) -> &'a[u8] {
        let start = node_number * 32;
        let end = start + 32;
        &self.raw_data[start..end]
    }

    pub fn nodes(&self) -> usize {
        return self.raw_data.len() / 32
    }

    pub fn leaf_count(&self) -> usize {
        let nodes = self.nodes();
        // next power of 2 below "nodes"
        // ...but we know that nodes should be "next power of 2 minus 1"
        assert!((nodes + 1) % 2 == 0);
        nodes + 1 / 2
    }
}


fn main() -> result::Result<(), io::Error> {
    let _ = read_cap("1of2.0");
    let mut part2 = File::open("1of2.1")?;
    //XXX BEWARE this is a lease thing with a share thing inside, see Issue #1
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part2.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(&pile_of_bytes);
    let s: Share = rdr.read_be().unwrap();
    let ueb_bytes = s.uri_ext;
    let the_ueb = read_ueb(&ueb_bytes)?;
    println!("UEB: {:?}", the_ueb);
    let taggy = tagged_hash(URI_TAG.as_bytes(), &ueb_bytes, 32);
    println!("UEB tagged hash?: {}", b2a(taggy));

    let start: usize = (s.data_offset as usize) + 12;
    let end: usize = start + s.data_size as usize;
    let data: &[u8] = &pile_of_bytes[start..end];

    assert!(data.len() == s.data_size as usize);

    let root: &[u8] = &s.crypttext_hash_tree[0..32];

    let cth = CryptTextHashTree{
        raw_data: &s.crypttext_hash_tree,
    };

    let mut gold_root: Option<UEB_Value> = None;
    for x in the_ueb.vals {
        match x {
            UEB_chunk::CryptTextRootHash(val) => {
                gold_root = Some(val);
                ()
            },
            _ => ()
        }
    }

    let chunks = data.chunks(8);
    let leaves: Vec<[u8; 32]> = chunks
        .map(|x| TahoeLeaf::hash(x))
        .collect();
    let merkle_tree = MerkleTree::<TahoeInside>::from_leaves(&leaves);

    // okay so this "Tahoe" thing is closer -- we do tagged
    // hashes. but of course Tahoe is weird, and we tag leaves
    // differently than "interior" nodes, but "the most advanced
    // merklet tree library for rust" doesn't support that notion?

    if let Some(root) = merkle_tree.root() {
        println!("merkle: {:?}", root);
        if let Some(gr) = gold_root {
            println!("      : {:?}", gr.pile_of_bytes);
        }
    }

    assert!(root.len() == 32);
    assert!(cth.node_hash(0).len() == 32);

    let first_leaf = &data[0..8];
    assert!(first_leaf.len() == 8);

    // do we know how to hash a leaf?
    let leaf0 = tagged_hash(b"allmydata_crypttext_segment_v1", first_leaf, 32);
    assert!(leaf0 == leaves[0]);
    // if this hash is correct, it should match what we have in the file
    // the "first leaf node" is index 7 (index 0 == root)
    let other = cth.node_hash(7);
    print!("leaf0: {:?}\nother: {:?}\n", leaf0, other);

    let hash: Vec<u8> = tagged_pair_hash(b"Merkle tree internal node", cth.node_hash(1), cth.node_hash(2));
    println!("\ncomputed: {:?}", hash);
    println!("    root: {:?}", root);
    assert!(root == hash.as_slice());

//b'Merkle tree internal node', a, b)
// def tagged_pair_hash(tag, val1, val2, truncate_to=None):
//     s = _SHA256d_Hasher(truncate_to)
//     s.update(netstring(tag))
//     s.update(netstring(val1))
//     s.update(netstring(val2))
//     return s.digest()
    // the "root hash" == hash of (1 + 2)


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

#[derive(Clone)]
pub struct TahoeLeaf {}

#[derive(Clone)]
pub struct TahoeInside {}

impl Hasher for TahoeLeaf {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {  //why not "Hash" as return type?
        //let mut engine = sha256d::Hash::engine();
        //engine.input(data);
        //sha256d::Hash::from_engine(engine).to_byte_array()
        let hash = tagged_hash(b"allmydata_crypttext_segment_v1", data, 32);
        let mut ret = [0; 32];
        ret.copy_from_slice(hash.as_slice());
        ret
    }
}

impl Hasher for TahoeInside {
    type Hash = [u8; 32];
    // we don't really want "generics, of u32 or str" etc we can just
    // add those as "things your Trait needs ot have"? is that the pattern?

    fn hash(data: &[u8]) -> [u8; 32] {  //why not "Hash" as return type?
        let hash = tagged_hash(b"Merkle tree internal node", data, 32);
        println!("inside hash {:?}", hash);
        let mut ret = [0; 32];
        ret.copy_from_slice(hash.as_slice());
        ret
    }
}


pub fn tagged_pair_hash(tag: &[u8], val0: &[u8], val1: &[u8]) -> Vec<u8> {
    let mut engine = sha256d::Hash::engine();
    engine.input(&netstring(tag));
    engine.input(&netstring(val0));
    engine.input(&netstring(val1));
    sha256d::Hash::from_engine(engine).to_byte_array()[0..32].to_vec()
}

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
    //format!("{}:{},", s.len(), std::str::from_utf8(s).unwrap()).into_bytes()

    // what Python does is output BYTES here, where we have some
    // number of ASCII-numeral bytes that represent the length, then a
    // ':' byte, and then 32 arbitrary bytes of key
    let tag = format!("{}:", s.len());
    // stuff two byte-sequences together; better way?
    [tag.as_bytes(), s, b","].concat()
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
