// TODO: from_str and compare(?)
// TODO: impl converter for Field

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MacAddr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
}

impl MacAddr {
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8) -> MacAddr {
        MacAddr {
            a: b1,
            b: b2,
            c: b3,
            d: b4,
            e: b5,
            f: b6,
        }
    }

    pub fn octets(&self) -> [u8; 6] {
        [self.a, self.b, self.c, self.d, self.e, self.f]
    }
}
