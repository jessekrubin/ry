const BB: u8 = b'b';
const TT: u8 = b't';
const NN: u8 = b'n';
const FF: u8 = b'f';
const RR: u8 = b'r';
const QU: u8 = b'"';
const BS: u8 = b'\\';
const UU: u8 = b'u';
const __: u8 = 0;

// A value of b'x' at index i means that byte i is escaped as "\x" in JSON.
// A value of 0 means that byte i is not escaped.
static ESCAPE: [u8; 256] = [
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

#[inline]
pub(crate) fn format_escaped_str(output: &mut Vec<u8>, value: &str) {
    output.push(b'"');
    format_escaped_str_contents(output, value);
    output.push(b'"');
}

/// JSON escape `&str`
///
/// Adapted from `serde_json`'s escape-table impl for an infallible sink (a vec).
///
/// ref: <https://github.com/serde-rs/json/blob/afdf6fc67247dd7fa4fcde1381e6ecc6bcc7a30e/src/ser.rs#L2079>
#[inline]
pub(crate) fn format_escaped_str_contents(output: &mut Vec<u8>, value: &str) {
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

    if !bytes.is_empty() {
        output.extend_from_slice(bytes);
    }
}
