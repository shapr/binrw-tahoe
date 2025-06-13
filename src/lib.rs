pub mod lib {

    use binrw::helpers::*;
    use binrw::io::*;
    use binrw::FilePtr;
    use binrw::*;
    use std::fmt::Debug;
    use std::io;
    use std::io::Write;

    #[derive(BinRead, PartialEq, Debug)]
    pub struct Share {
	pub lease_version: u32,
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
	pub uri_ext_offset: u32,
	// "+ 12" because we're inside a "lease" struct w/ 3x u32
	#[br(seek_before(SeekFrom::Start((uri_ext_offset + 12) as u64)))]
	pub uri_ext_size: u32,
	#[br(count=uri_ext_size)]
	pub uri_ext: Vec<u8>,
	//	uri_ext_size: FilePtr<u32, u32>,
	// #[br(value = uri_ext_size)]
	// uri_ugly_hack: u32,
	//	#[br(parse_with = FilePtr::parse(u32), seek_before(SeekFrom::Start(12 + uri_ext_offset)))]
	//	uri_ext_size: u32,
	//	#[br(count = *uri_ext_size)]
	//	uri_block: Vec<u8>,
	// uri_block: FilePtr<u32, Vec<u8>>,
	// data starts now!
	//	#[br(big, count = data_size)]
	//	share_data: Vec<u8>,
    }

    fn bytes_to_int(v: &Vec<u8>) -> u32 {
	let s = String::try_from(v.clone()).expect("not what you wanted");
	println!("Here's the s {}", s.clone());
	let byte_count = s.parse().expect("wasn't an ASCII integer");
	return byte_count;
    }

    #[derive(BinRead, PartialEq, Debug)]
    pub struct UEB {
	#[br(parse_with = until_eof)]
	vals: Vec<UEB_chunk>,
    }

    #[derive(BinRead, PartialEq, Debug)]
    pub enum UEB_chunk {
	#[br(magic = b"codec_name:")]
	CodecName(UEB_Value),
	#[br(magic = b"codec_params:")]
	CodecParams(UEB_Value),
	#[br(magic = b"crypttext_hash:")]
	CryptText(UEB_Value),
	#[br(magic = b"crypttext_root_hash:")]
	CryptTextRootHash(UEB_Value),
	#[br(magic = b"needed_shares:")]
	NeededShares(UEB_Value),
	#[br(magic = b"num_segments:")]
	NumSegments(UEB_Value),
	#[br(magic = b"segment_size:")]
	SegmentSize(UEB_Value),
	#[br(magic = b"share_root_hash:")]
	ShareRootHash(UEB_Value),
	#[br(magic = b"tail_codec_params:")]
	TailCodecParams(UEB_Value),
	#[br(magic = b"total_shares:")]
	TotalShares(UEB_Value),
	Debug([u8; 5]),
    }

    #[derive(BinRead, PartialEq, Debug)]
    pub struct UEB_Value {
	// can I use the parse helper to compose until_exclusive with something : Vec<u8> -> String ?
	// maybe?
	#[br(parse_with = until_exclusive(|&byte| byte == b':'))]
	count_of_bytes: Vec<u8>, // gotta convert to an ASCII number, and then back to a value
	#[br(count = bytes_to_int(&count_of_bytes))]
	pile_of_bytes: Vec<u8>,
	trailing_comma: u8,
    }
    // #[binrw::parser(reader: r, endian)]
    // fn custom_parser(v0: u8, v1: i16) -> binrw::BinResult<u32> {
    //     // turn Vec<u8> into u32
    //     Ok(upcoming_bytes_count)
    // }

    /* guess:
    split on commas for each "value"
    split on colons inside the "value"

    Wait, no... the hashes are binary values, and thus could have a ',' in their value, so we have to continue with binary parsing!
    That is, we must read from the start of the URI extension block up to the number between two colons, and then consume that many bytes for this named value.
    We cannot split on commas because the hashes could contain a comma!
    */

    // fn foo<Ret, T, Arg, Reader>(r: &mut Reader, e: Endian, n: Arg) -> BinResult<UEB_Value> {
    //	return (|| Ok(Debug(b"12345")));
    // }

    pub fn print_hello_world() {
	let _ = io::stdout().write_all(b"Hello, world!\n");
    }
}
