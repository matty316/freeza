pub(crate) fn is_digit(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

pub(crate) fn is_alpha(c: u8) -> bool {
    b'a' <= c && c <= b'z' || b'A' <= c && c <= b'Z' || c == b'_'
}