//! JSON string escaping
//!
//! REFERENCES:
//!   - <https://lemire.me/blog/2025/04/13/detect-control-characters-quotes-and-backslashes-efficiently-using-swar/>
//!   - v8
//!     - <https://v8.dev/blog/json-stringify>
//!     - <https://source.chromium.org/chromium/chromium/src/+/main:v8/src/json/json-stringifier.cc;drc=1645281bbd1b183a252835d376166bd210135bbe;l=3353>

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const UU: u8 = b'u'; // \x00...\x1F except the ones above
const __: u8 = 0;

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
pub(crate) static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU, // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];

// #[inline]
// pub(crate) fn format_escaped_str(output: &mut Vec<u8>, value: &str) {
//     output.push(b'"');
//     format_escaped_str_contents(output, value);
//     output.push(b'"');
// }

/// JSON escape `&str`
///
/// Adapted from `serde_json`'s escape-table impl for writing to a vec
///
/// ref: <https://github.com/serde-rs/json/blob/afdf6fc67247dd7fa4fcde1381e6ecc6bcc7a30e/src/ser.rs#L2079>
#[inline]
pub fn escape_into_scalar(output: &mut Vec<u8>, value: &str) {
    let mut bytes = value.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let string_run = &bytes[..i];
        let byte = bytes[i];
        let rest = &bytes[i + 1..];

        let escape = ESCAPE[byte as usize];

        i += 1;
        if escape == 0 {
            continue;
        }

        bytes = rest;
        i = 0;

        if !string_run.is_empty() {
            output.extend_from_slice(string_run);
        }

        write_escape(output, byte, escape);
    }

    if !bytes.is_empty() {
        output.extend_from_slice(bytes);
    }
}

/// write the escape seq for `byte`; `escape` is its (non-zero) `ESCAPE` entry
#[inline]
fn write_escape(output: &mut Vec<u8>, byte: u8, escape: u8) {
    if escape == UU {
        const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
        output.extend_from_slice(&[
            b'\\',
            b'u',
            b'0',
            b'0',
            HEX_DIGITS[(byte >> 4) as usize],
            HEX_DIGITS[(byte & 0x0f) as usize],
        ]);
    } else {
        output.extend_from_slice(&[b'\\', escape]);
    }
}

// ----------------------------------------------------------------------------
// SWAR
// ----------------------------------------------------------------------------

/// SWAR u64 impl
///
/// REF: <https://lemire.me/blog/2025/04/13/detect-control-characters-quotes-and-backslashes-efficiently-using-swar/>
///
/// ```cpp
/// bool has_json_escapable_byte(uint64_t x) {
///   uint64_t is_ascii = 0x8080808080808080ULL & ~x;
///   uint64_t xor2 = x ^ 0x0202020202020202ULL;
///   uint64_t lt32_or_eq34 = xor2 - 0x2121212121212121ULL;
///   uint64_t sub92 = x ^ 0x5C5C5C5C5C5C5C5CULL;
///   uint64_t eq92 = (sub92 - 0x0101010101010101ULL);
///   return ((lt32_or_eq34 | eq92) & is_ascii) != 0;
/// }
/// ```
#[inline]
const fn json_escapable_mask(x: u64) -> u64 {
    let is_ascii = 0x8080_8080_8080_8080 & !x;
    let xor2 = x ^ 0x0202_0202_0202_0202;
    let lt32_or_eq34 = xor2.wrapping_sub(0x2121_2121_2121_2121);
    let sub92 = x ^ 0x5C5C_5C5C_5C5C_5C5C;
    let eq92 = sub92.wrapping_sub(0x0101_0101_0101_0101);
    (lt32_or_eq34 | eq92) & is_ascii
}

/// JSON escape `&str` (swar)
///
/// flat on purpose; splitting it into helpers benched slower
///
/// # Panics
///
/// if the second `split_first` below fails which it really should not (wenodis)
#[inline]
pub fn escape_into_swar_u64(output: &mut Vec<u8>, value: &str) {
    let mut rest = value.as_bytes();

    loop {
        // find next byte to escape and return its index (if any)
        // but also update the remaining (todo) slice
        let mut unscanned = rest;
        let hit_ix = 'found: {
            while let Some((chunk, tail)) = unscanned.split_first_chunk::<8>() {
                let mask = json_escapable_mask(u64::from_le_bytes(*chunk));
                if mask != 0 {
                    let chunk_ix = (mask.trailing_zeros() / 8) as usize;
                    break 'found Some(rest.len() - unscanned.len() + chunk_ix);
                }
                unscanned = tail;
            }
            // remaining tail lt 8 bytes
            unscanned
                .iter()
                .position(|&byte| ESCAPE[byte as usize] != 0)
                .map(|i| rest.len() - unscanned.len() + i)
        };

        // we got a hit!
        if let Some(hit_ix) = hit_ix {
            // split at the hit and escape the byte at the front of the tail
            let (no_esc_run, tail) = rest.split_at(hit_ix);
            // and againt split bc we need the first byte separately for escaping
            let Some((&byte, tail)) = tail.split_first() else {
                unreachable!("wenodis: inbounds")
            };
            output.extend_from_slice(no_esc_run);
            write_escape(output, byte, ESCAPE[byte as usize]);
            rest = tail;
            // contiguous escapable bytes (eg `\r\n` (stupid windows))
            while let Some((&byte, tail)) = rest.split_first() {
                let escape = ESCAPE[byte as usize];
                if escape == 0 {
                    break;
                }
                write_escape(output, byte, escape);
                rest = tail;
            }
        } else {
            output.extend_from_slice(rest);
            return;
        }
    }
}
