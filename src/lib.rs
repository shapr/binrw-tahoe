pub mod lib {

    use binrw::io::*;
    use binrw::FilePtr;
    use binrw::*;
    use binrw::helpers::*;
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
	uri_ext_offset: u32,
        // "+ 12" because we're inside a "lease" struct w/ 3x u32
        #[br(seek_before(SeekFrom::Start((uri_ext_offset + 12) as u64)))]
        uri_ext_size: u32,

        ///v == NoMore))]
//        #[br(parse_with = until_exclusive(|&UebValue v| true))]
        data: Vec<UebValue>,


//        #[br(count=uri_ext_size)]
//        pub uri_ext: Vec<u8>,


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

    // use nom::*;
    /*
    struct UriExtension {
    /// codec_name must be "crs" as of 2025
    codec_name: String,
    /// segment size, needed shares, total shares
    codec_params: (usize, usize, usize),
    /// segment size, needed shares, total shares
    tail_codec_params: (usize, usize, usize),
    /// application size in bytes
    size: usize,
    /// segment size in bytes
    segment_size: usize,
    segment_count: usize,
    share_count_required: usize,
    share_count_total: usize,
    crypt_text_hash: Vec<u8>,
    crypt_text_root_hash: Vec<u8>,
    share_root_hash: Vec<u8>,
    }
    */

    // demo input "codec_name:3:crs,codec_params:5:8-1-2,"
    /* guess:
    split on commas for each "value"
    split on colons inside the "value"

    Wait, no... the hashes are binary values, and thus could have a ',' in their value, so we have to continue with binary parsing!
    That is, we must read from the start of the URI extension block up to the number between two colons, and then consume that many bytes for this named value.
    We cannot split on commas because the hashes could contain a comma!
    */

    #[derive(PartialEq, Debug)]
    struct UebValue {
	name: String,
	byte_count: usize,
	value: Vec<u8>,
    }

    impl<T> BinRead for UebValue
    where
        for <'a> T: BinRead<Args<'a> = ()>,
//        BinRead<Args> = (),
    {
        type Args<'a> = ();

        fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: UebValue::Args) -> BinResult<Self> {
            return Err(AssertFail(0, "foo"))
        }
    }

/// read stuff sequentiallly ... return the sentinel if we're done


    /*
      What about using the names as "magic numbers" as found in the first few bytes of a file?
    */

    // #[derive(BinRead)]
    // enum URI_Extension {
    //     #[brw(big, magic = b"share_root_hash:")]
    //     Share_root_hash { Vec<u8> },
    //     #[br(magic(0u8))] Rect {
    //	left: i16, top: i16, right: i16, bottom: i16
    //     },
    //     #[br(magic(1u8))]
    //     Oval { origin: Point, rx: u8, ry: u8 }
    // }

    pub fn print_hello_world() {
	let _ = io::stdout().write_all(b"Hello, world!\n");
    }
}
