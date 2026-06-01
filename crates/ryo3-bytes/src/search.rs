#[cfg(feature = "memchr")]
pub(crate) fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    ::memchr::memchr(needle, haystack)
}

#[cfg(feature = "memchr")]
pub(crate) fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    ::memchr::memrchr(needle, haystack)
}

#[cfg(feature = "memchr")]
pub(crate) fn memchr_iter(needle: u8, haystack: &[u8]) -> ::memchr::Memchr<'_> {
    ::memchr::memchr_iter(needle, haystack)
}

#[cfg(not(feature = "memchr"))]
pub(crate) fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}

#[cfg(not(feature = "memchr"))]
pub(crate) fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == needle)
}

#[cfg(not(feature = "memchr"))]
pub(crate) fn memchr_iter(needle: u8, haystack: &[u8]) -> impl Iterator<Item = usize> + '_ {
    haystack
        .iter()
        .enumerate()
        .filter_map(move |(i, &b)| (b == needle).then_some(i))
}

pub(crate) fn find_subslice(buf: &[u8], needle: &[u8]) -> Option<usize> {
    match needle {
        [] => Some(0),
        [byte] => memchr(*byte, buf),
        _ => buf
            .windows(needle.len())
            .position(|window| window == needle),
    }
}

pub(crate) fn rfind_subslice(buf: &[u8], needle: &[u8]) -> Option<usize> {
    match needle {
        [] => Some(buf.len()),
        [byte] => memrchr(*byte, buf),
        _ => buf
            .windows(needle.len())
            .rposition(|window| window == needle),
    }
}
