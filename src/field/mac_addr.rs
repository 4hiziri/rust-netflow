// TODO: from_str and compare(?)
// TODO: impl converter for Field

#[derive(Debug, Copy, Clone)]
pub struct MacAddr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
}

impl MacAddr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> MacAddr {
        MacAddr {
            a: a,
            b: b,
            c: c,
            d: d,
            e: e,
            f: f,
        }
    }

    pub fn octets(&self) -> [u8; 6] {
        [self.a, self.b, self.c, self.d, self.e, self.f]
    }
}
