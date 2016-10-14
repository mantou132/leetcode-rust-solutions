pub fn isupper(c: u8) -> bool {
    match c {
        b'A' ... b'Z' => true,
        _ => false
    }
}
