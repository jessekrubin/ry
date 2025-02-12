#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PyRegexOptions {
    case_insensitive: bool,
    crlf: bool,
    dot_matches_new_line: bool,
    ignore_whitespace: bool,
    line_terminator: u8,
    multi_line: bool,
    octal: bool,
    size_limit: Option<usize>,
    swap_greed: bool,
    unicode: bool,
}

impl Default for PyRegexOptions {
    fn default() -> Self {
        PyRegexOptions {
            case_insensitive: false,
            crlf: false,
            dot_matches_new_line: false,
            ignore_whitespace: false,
            line_terminator: b'\n',
            multi_line: false,
            octal: false,
            size_limit: None,
            swap_greed: false,
            unicode: true,
        }
    }
}
