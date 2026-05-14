mod utils {
    #[cfg(feature = "memchr")]
    pub(super) fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
        ::memchr::memchr(needle, haystack)
    }

    #[cfg(feature = "memchr")]
    pub(super) fn memchr_iter(needle: u8, haystack: &[u8]) -> ::memchr::Memchr<'_> {
        ::memchr::memchr_iter(needle, haystack)
    }

    #[cfg(not(feature = "memchr"))]
    pub(super) fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
        haystack.iter().position(|&b| b == needle)
    }

    #[cfg(not(feature = "memchr"))]
    pub(super) fn memchr_iter(needle: u8, haystack: &[u8]) -> impl Iterator<Item = usize> + '_ {
        haystack
            .iter()
            .enumerate()
            .filter_map(move |(i, &b)| (b == needle).then_some(i))
    }
}

pub(crate) enum ReplaceBytes {
    Unchanged,
    Replaced(Vec<u8>),
}

struct ReplaceEmptyOld<'a> {
    new: &'a [u8],
    count: usize,
}

impl ReplaceEmptyOld<'_> {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        let insertion_count = self.count.min(buf.len() + 1);
        let mut out = Vec::with_capacity(
            buf.len()
                .saturating_add(self.new.len().saturating_mul(insertion_count)),
        );
        let mut inserted = 0usize;

        for &byte in buf {
            if inserted < insertion_count {
                out.extend_from_slice(self.new);
                inserted += 1;
            }
            out.push(byte);
        }

        if inserted < insertion_count {
            out.extend_from_slice(self.new);
        }

        ReplaceBytes::Replaced(out)
    }
}

struct Byte2Byte {
    old: u8,
    new: u8,
    count: usize,
}

impl Byte2Byte {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        let Some(first_match) = utils::memchr(self.old, buf) else {
            return ReplaceBytes::Unchanged;
        };

        let mut out = buf.to_vec();
        let mut replaced = 0usize;

        for byte in &mut out[first_match..] {
            if *byte == self.old {
                *byte = self.new;
                replaced += 1;
                if replaced == self.count {
                    break;
                }
            }
        }
        ReplaceBytes::Replaced(out)
    }
}

struct Byte2Many<'a> {
    old: u8,
    new: &'a [u8],
    count: usize,
}

impl Byte2Many<'_> {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        let match_count = utils::memchr_iter(self.old, buf).take(self.count).count();
        if match_count == 0 {
            return ReplaceBytes::Unchanged;
        }

        let mut out = Vec::with_capacity(replacement_capacity(
            buf.len(),
            1,
            self.new.len(),
            match_count,
        ));
        let mut start = 0usize;

        for index in utils::memchr_iter(self.old, buf).take(self.count) {
            out.extend_from_slice(&buf[start..index]);
            out.extend_from_slice(self.new);
            start = index + 1;
        }

        out.extend_from_slice(&buf[start..]);
        ReplaceBytes::Replaced(out)
    }
}

struct ReplaceUno<'a> {
    old: &'a [u8],
    new: &'a [u8],
}

impl ReplaceUno<'_> {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        let Some(index) = find_subslice(buf, self.old) else {
            return ReplaceBytes::Unchanged;
        };

        let mut out = Vec::with_capacity(replacement_capacity(
            buf.len(),
            self.old.len(),
            self.new.len(),
            1,
        ));
        out.extend_from_slice(&buf[..index]);
        out.extend_from_slice(self.new);
        out.extend_from_slice(&buf[index + self.old.len()..]);
        ReplaceBytes::Replaced(out)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Replacement<'a> {
    None,
    Byte(u8),
    Bytes(&'a [u8]),
}

impl<'a> From<&'a [u8]> for Replacement<'a> {
    fn from(value: &'a [u8]) -> Self {
        match value {
            [] => Self::None,
            [byte] => Self::Byte(*byte),
            _ => Self::Bytes(value),
        }
    }
}

struct RemoveByte {
    old: u8,
    count: usize,
}

impl RemoveByte {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        let match_count = utils::memchr_iter(self.old, buf).take(self.count).count();
        if match_count == 0 {
            return ReplaceBytes::Unchanged;
        }

        let mut out = Vec::with_capacity(buf.len().saturating_sub(match_count));
        let mut start = 0usize;

        for index in utils::memchr_iter(self.old, buf).take(self.count) {
            out.extend_from_slice(&buf[start..index]);
            start = index + 1;
        }

        out.extend_from_slice(&buf[start..]);
        ReplaceBytes::Replaced(out)
    }
}

struct EqualLengthSubstring<'a> {
    old: &'a [u8],
    new: &'a [u8],
    count: usize,
}

impl EqualLengthSubstring<'_> {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        let match_count = count_subslice_matches_limit(buf, self.old, self.count);
        if match_count == 0 {
            return ReplaceBytes::Unchanged;
        }

        let mut out = Vec::with_capacity(buf.len());
        let mut start = 0usize;
        let mut remainder = buf;
        let mut offset = 0usize;
        let mut replaced = 0usize;

        while replaced < self.count {
            let Some(index) = find_subslice(remainder, self.old) else {
                break;
            };
            let absolute_index = offset + index;
            out.extend_from_slice(&buf[start..absolute_index]);
            out.extend_from_slice(self.new);
            start = absolute_index + self.old.len();
            remainder = &buf[start..];
            offset = start;
            replaced += 1;
        }
        out.extend_from_slice(&buf[start..]);
        ReplaceBytes::Replaced(out)
    }
}

struct ReplaceMany<'a> {
    old: &'a [u8],
    new: Replacement<'a>,
    count: usize,
}

fn replace_substrings(buf: &[u8], old: &[u8], new: &[u8], count: usize) -> ReplaceBytes {
    let match_count = count_subslice_matches_limit(buf, old, count);
    if match_count == 0 {
        return ReplaceBytes::Unchanged;
    }

    let mut out = Vec::with_capacity(replacement_capacity(
        buf.len(),
        old.len(),
        new.len(),
        match_count,
    ));
    let mut start = 0usize;
    let mut remainder = buf;
    let mut offset = 0usize;
    let mut replaced = 0usize;

    while replaced < count {
        let Some(index) = find_subslice(remainder, old) else {
            break;
        };
        let absolute_index = offset + index;
        out.extend_from_slice(&buf[start..absolute_index]);
        out.extend_from_slice(new);
        start = absolute_index + old.len();
        remainder = &buf[start..];
        offset = start;
        replaced += 1;
    }

    out.extend_from_slice(&buf[start..]);
    ReplaceBytes::Replaced(out)
}

impl ReplaceMany<'_> {
    fn apply(&self, buf: &[u8]) -> ReplaceBytes {
        match self.new {
            Replacement::None => replace_substrings(buf, self.old, &[], self.count),
            Replacement::Byte(new) => replace_substrings(buf, self.old, &[new], self.count),
            Replacement::Bytes(new) => replace_substrings(buf, self.old, new, self.count),
        }
    }
}

enum ReplacePlan<'a> {
    NoOp,
    EmptyOld(ReplaceEmptyOld<'a>),
    SingleByteToSingleByte(Byte2Byte),
    SingleByteToMany(Byte2Many<'a>),
    RemoveByte(RemoveByte),
    ReplaceUno(ReplaceUno<'a>),
    EqualLengthSubstring(EqualLengthSubstring<'a>),
    ReplaceSubstrings(ReplaceMany<'a>),
}

impl<'a> ReplacePlan<'a> {
    fn select(buf: &[u8], old: &'a [u8], new: &'a [u8], count: isize) -> Self {
        if count == 0 || (!old.is_empty() && (buf.is_empty() || old.len() > buf.len())) {
            return Self::NoOp;
        }

        let c = usize::try_from(count).unwrap_or(usize::MAX);
        if old.is_empty() {
            if new.is_empty() {
                return Self::NoOp;
            }
            return Self::EmptyOld(ReplaceEmptyOld { new, count: c });
        }

        let new_rep = Replacement::from(new);
        // dis ugly af ~ what was i doing last night
        match Replacement::from(old) {
            Replacement::None => Self::NoOp,
            Replacement::Byte(old) => match new_rep {
                Replacement::None => Self::RemoveByte(RemoveByte { old, count: c }),
                Replacement::Byte(new) => {
                    if old == new {
                        Self::NoOp
                    } else {
                        Self::SingleByteToSingleByte(Byte2Byte { old, new, count: c })
                    }
                }
                Replacement::Bytes(new) => Self::SingleByteToMany(Byte2Many { old, new, count: c }),
            },
            Replacement::Bytes(old) => match new_rep {
                Replacement::None => Self::ReplaceSubstrings(ReplaceMany {
                    old,
                    new: Replacement::None,
                    count: c,
                }),
                Replacement::Byte(new) => Self::ReplaceSubstrings(ReplaceMany {
                    old,
                    new: Replacement::Byte(new),
                    count: c,
                }),
                Replacement::Bytes(new) => {
                    if old == new {
                        Self::NoOp
                    } else if c == 1 {
                        Self::ReplaceUno(ReplaceUno { old, new })
                    } else if old.len() == new.len() {
                        Self::EqualLengthSubstring(EqualLengthSubstring { old, new, count: c })
                    } else {
                        Self::ReplaceSubstrings(ReplaceMany {
                            old,
                            new: Replacement::Bytes(new),
                            count: c,
                        })
                    }
                }
            },
        }
    }

    fn apply(self, buf: &[u8]) -> ReplaceBytes {
        match self {
            Self::NoOp => ReplaceBytes::Unchanged,
            Self::EmptyOld(plan) => plan.apply(buf),
            Self::SingleByteToSingleByte(plan) => plan.apply(buf),
            Self::SingleByteToMany(plan) => plan.apply(buf),
            Self::RemoveByte(plan) => plan.apply(buf),
            Self::ReplaceUno(plan) => plan.apply(buf),
            Self::EqualLengthSubstring(plan) => plan.apply(buf),
            Self::ReplaceSubstrings(plan) => plan.apply(buf),
        }
    }
}

pub(crate) fn replace_bytes(buf: &[u8], old: &[u8], new: &[u8], count: isize) -> ReplaceBytes {
    ReplacePlan::select(buf, old, new, count).apply(buf)
}

fn find_subslice(buf: &[u8], needle: &[u8]) -> Option<usize> {
    match needle {
        [] => Some(0),
        [byte] => utils::memchr(*byte, buf),
        _ => buf
            .windows(needle.len())
            .position(|window| window == needle),
    }
}

fn count_subslice_matches_limit(buf: &[u8], needle: &[u8], count: usize) -> usize {
    let mut n_replacements = 0usize;
    let mut remainder = buf;

    while n_replacements < count {
        let Some(index) = find_subslice(remainder, needle) else {
            break;
        };
        remainder = &remainder[index + needle.len()..];
        n_replacements += 1;
    }

    n_replacements
}

fn replacement_capacity(
    original_len: usize,
    old_len: usize,
    new_len: usize,
    nmatches: usize,
) -> usize {
    if new_len >= old_len {
        original_len.saturating_add((new_len - old_len).saturating_mul(nmatches))
    } else {
        original_len.saturating_sub((old_len - new_len).saturating_mul(nmatches))
    }
}

#[cfg(test)]
mod tests {
    use super::{ReplaceBytes, replace_bytes};

    #[test]
    fn replace_bytes_replaces_non_overlapping_matches() {
        let replaced = replace_bytes(b"aaaa", b"aa", b"b", -1);
        match replaced {
            ReplaceBytes::Replaced(bytes) => assert_eq!(bytes.as_slice(), b"bb"),
            ReplaceBytes::Unchanged => panic!("expected replacement"),
        }
    }

    #[test]
    fn replace_bytes_honors_count_for_empty_old() {
        let replaced = replace_bytes(b"abc", b"", b"-", 2);
        match replaced {
            ReplaceBytes::Replaced(bytes) => assert_eq!(bytes.as_slice(), b"-a-bc"),
            ReplaceBytes::Unchanged => panic!("expected replacement"),
        }
    }

    #[test]
    fn replace_bytes_returns_unchanged_when_no_replacement_occurs() {
        assert!(matches!(
            replace_bytes(b"abc", b"x", b"y", -1),
            ReplaceBytes::Unchanged
        ));
        assert!(matches!(
            replace_bytes(b"abc", b"a", b"b", 0),
            ReplaceBytes::Unchanged
        ));
    }

    #[test]
    fn replace_bytes_returns_replaced_when_output_matches_input() {
        assert!(matches!(
            replace_bytes(b"abc", b"", b"", -1),
            ReplaceBytes::Unchanged
        ));
        assert!(matches!(
            replace_bytes(b"abc", b"a", b"a", -1),
            ReplaceBytes::Unchanged
        ));
    }
}
