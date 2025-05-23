// use nom::*;

struct URI_Extension {
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
// demo input "codec_name:3:crs,codec_params:5:8-1-2,"
/* guess:
split on commas for each "value"
split on colons inside the "value"

Wait, no... the hashes are binary values, and thus could have a ',' in their value, so we have to continue with binary parsing!
That is, we must read from the start of the URI extension block up to the number between two colons, and then consume that many bytes for this named value.
We cannot split on commas because the hashes could contain a comma!
*/

struct UEB_Value {
    name: String,
    byte_count: usize,
    value: Vec<u8>,
}
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
