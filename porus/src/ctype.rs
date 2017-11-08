pub fn isupper(c: u8) -> bool {
    match c {
        b'A' ... b'Z' => true,
        _ => false,
    }
}

pub fn islower(c: u8) -> bool {
    match c {
        b'a' ... b'z' => true,
        _ => false,
    }
}

pub fn isalpha(c: u8) -> bool {
    match c {
        b'A' ... b'Z' => true,
        b'a' ... b'z' => true,
        _ => false,
    }
}

pub fn isdigit(c: u8) -> bool {
    match c {
        b'0' ... b'9' => true,
        _ => false,
    }
}

pub fn isxdigit(c: u8) -> bool {
    match c {
        b'A' ... b'F' => true,
        b'a' ... b'f' => true,
        b'0' ... b'9' => true,
        _ => false,
    }
}

pub fn isalnum(c: u8) -> bool {
    match c {
        b'A' ... b'Z' => true,
        b'a' ... b'z' => true,
        b'0' ... b'9' => true,
        _ => false,
    }
}

pub fn ispunct(c: u8) -> bool {
    match c {
        b'!' ... b'/' => true,
        b':' ... b'@' => true,
        b'[' ... b'`' => true,
        b'{' ... b'~' => true,
        _ => false,
    }
}

pub fn isgraph(c: u8) -> bool {
    match c {
        b'!' ... b'~' => true,
        _ => false,
    }
}

pub fn isprint(c: u8) -> bool {
    match c {
        b' ' ... b'~' => true,
        _ => false,
    }
}

pub fn isblank(c: u8) -> bool {
    match c {
        b' ' | b'\t' => true,
        _ => false,
    }
}

pub fn isnewline(c: u8) -> bool {
    match c {
        b'\n' => true,
        _ => false,
    }
}

pub fn isspace(c: u8) -> bool {
    match c {
        b' ' | b'\t' ... b'\r' => true,
        _ => false
    }
}

pub fn iscntrl(c: u8) -> bool {
    match c {
        0 ... 0x1f => true,
        0x7f => true,
        _ => false,
    }
}
