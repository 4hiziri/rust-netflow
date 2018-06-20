use super::MacAddr;
use std::collections::HashSet;
use std::convert::From;
use std::net::{Ipv4Addr, Ipv6Addr};
use util::{
    take_u128, take_u16, take_u32, take_u64, u128_to_bytes, u16_to_bytes, u32_to_bytes,
    u64_to_bytes,
};

// research types
// 1. flexible length num, length = N bytes
// 2. fixed length num, length = 1, 2, 3, 4
// 3. byte array
// 4. ipv4 address
// 5. ipv6 address
// 6. mac address
// 7. string

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue {
    NumField(UInt),
    ByteArray(Vec<u8>),
    Ipv4Addr(Ipv4Addr),
    Ipv6Addr(Ipv6Addr),
    MacAddr(MacAddr),
    String(String),
    Unknown(Vec<u8>),
}

// TODO: use import!
lazy_static! {
    static ref NUM_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set.insert(7);
        set.insert(10);
        set.insert(13);
        set.insert(14);
        set.insert(16);
        set.insert(17);
        set.insert(19);
        set.insert(20);
        set.insert(21);
        set.insert(22);
        set.insert(23);
        set.insert(24);
        set.insert(25);
        set.insert(26);
        set.insert(29);
        set.insert(30);
        set.insert(31);
        set.insert(32);
        set.insert(33);
        set.insert(34);
        set.insert(35);
        set.insert(36);
        set.insert(37);
        set.insert(38);
        set.insert(39);
        set.insert(40);
        set.insert(41);
        set.insert(42);
        set.insert(43);
        set.insert(46);
        set.insert(47);
        set.insert(48);
        set.insert(49);
        set.insert(50);
        set.insert(51);
        set.insert(52);
        set.insert(53);
        set.insert(54);
        set.insert(55);
        set.insert(58);
        set.insert(59);
        set.insert(60);
        set.insert(61);
        set.insert(64);
        set.insert(65);
        set.insert(66);
        set.insert(67);
        set.insert(68);
        set.insert(69);
        set.insert(70);
        set.insert(71);
        set.insert(72);
        set.insert(73);
        set.insert(74);
        set.insert(75);
        set.insert(76);
        set.insert(77);
        set.insert(78);
        set.insert(79);
        set.insert(85);
        set.insert(86);
        set.insert(87);
        set.insert(88);
        set.insert(89);
        set.insert(91);
        set.insert(92);
        set.insert(93);
        set.insert(94);
        set.insert(96);
        set.insert(98);
        set.insert(99);
        set.insert(100);
        set.insert(102);
        set.insert(103);
        set.insert(104);

        set
    };
    static ref BYTES_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(90);

        set
    };
    static ref IPV4_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(8);
        set.insert(12);
        set.insert(15);
        set.insert(18);
        set.insert(44);
        set.insert(45);

        set
    };
    static ref IPV6_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(27);
        set.insert(28);
        set.insert(62);
        set.insert(63);

        set
    };
    static ref MACADDR_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(56);
        set.insert(57);
        set.insert(80);
        set.insert(81);

        set
    };
    static ref STRING_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(82);
        set.insert(83);
        set.insert(84);

        set
    };
    static ref BITS_ID: HashSet<u16> = {
        let mut set = HashSet::new();
        set.insert(95);

        set
    };
}

impl FieldValue {
    fn is_num_field(type_id: u16) -> bool {
        NUM_ID.contains(&type_id)
    }

    fn is_bytes_field(type_id: u16) -> bool {
        BYTES_ID.contains(&type_id)
    }

    fn is_ipv4_field(type_id: u16) -> bool {
        IPV4_ID.contains(&type_id)
    }

    fn is_ipv6_field(type_id: u16) -> bool {
        IPV6_ID.contains(&type_id)
    }

    fn is_mac_field(type_id: u16) -> bool {
        MACADDR_ID.contains(&type_id)
    }

    fn is_string_field(type_id: u16) -> bool {
        STRING_ID.contains(&type_id)
    }

    fn is_bits_field(type_id: u16) -> bool {
        BITS_ID.contains(&type_id)
    }

    pub fn new(type_id: u16, value: &[u8]) -> FieldValue {
        if FieldValue::is_num_field(type_id) {
            FieldValue::NumField(UInt::from_bytes(value))
        } else if FieldValue::is_bytes_field(type_id) {
            FieldValue::ByteArray(value.to_vec())
        } else if FieldValue::is_ipv4_field(type_id) {
            let ip = take_u32(&value).unwrap().1;
            FieldValue::Ipv4Addr(Ipv4Addr::from(ip))
        } else if FieldValue::is_ipv6_field(type_id) {
            FieldValue::Ipv6Addr(Ipv6Addr::from(take_u128(value).unwrap().1))
        } else if FieldValue::is_mac_field(type_id) {
            FieldValue::MacAddr(MacAddr::new(
                value[0], value[1], value[2], value[3], value[4], value[5],
            ))
        } else if FieldValue::is_string_field(type_id) {
            FieldValue::String(String::from_utf8(value.to_vec()).unwrap())
        } else if FieldValue::is_bits_field(type_id) {
            FieldValue::NumField(UInt::from_bytes(value))
        } else {
            FieldValue::ByteArray(value.to_vec())
        }
    }

    pub fn to_bytes(&self, length: u16) -> Vec<u8> {
        match &self {
            FieldValue::NumField(uint) => uint.to_bytes(length),
            FieldValue::ByteArray(array) => array.to_vec(),
            FieldValue::Ipv4Addr(ipv4) => ipv4.octets().to_vec(),
            FieldValue::Ipv6Addr(ipv6) => ipv6.octets().to_vec(),
            FieldValue::MacAddr(mac) => mac.octets().to_vec(),
            FieldValue::String(s) => s.clone().into_bytes(), // TODO: I think it will be ok, but should check byte order.
            FieldValue::Unknown(array) => array.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UInt {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    UIntFlex(Vec<u8>),
}

/// use for representing u-int field value
/// every field can have various bit-length
/// so, field must be able to accept such config
impl UInt {
    fn fit_vec(len: usize, val: u8, bytes: &[u8]) -> Vec<u8> {
        let mut vec = Vec::from(bytes);

        if bytes.len() < len {
            for _ in 0..len - bytes.len() {
                vec.push(val);
            }
        }

        vec
    }

    // convert uint from [u8] as BigEndian
    pub fn from_bytes(bytes: &[u8]) -> UInt {
        let len = bytes.len();

        if len == 1 {
            UInt::UInt8(bytes[0])
        } else if len == 2 {
            UInt::UInt16(take_u16(&bytes).unwrap().1)
        } else if len > 2 && len <= 4 {
            UInt::UInt32(take_u32(&UInt::fit_vec(4, 0, bytes)).unwrap().1)
        } else if len > 4 && len <= 8 {
            UInt::UInt64(take_u64(&UInt::fit_vec(8, 0, bytes)).unwrap().1)
        } else if len > 8 && len <= 16 {
            UInt::UInt128(take_u128(&UInt::fit_vec(16, 0, bytes)).unwrap().1)
        } else {
            UInt::UIntFlex(bytes.to_vec())
        }
    }

    fn validate_length(buf: &[u8], length: usize) -> Vec<u8> {
        let buf_len = buf.len();

        if buf_len > length {
            buf[(buf_len - length)..].to_vec()
        } else if buf_len < length {
            let mut bytes = Vec::with_capacity(length);

            for _ in 0..(length - buf_len) {
                bytes.push(0); // FIXME: should return error
            }

            bytes.append(&mut buf.to_vec());

            bytes
        } else {
            buf.to_vec()
        }
    }

    pub fn to_bytes(&self, length: u16) -> Vec<u8> {
        let length = length as usize;

        // TODO: check length and type matching, but need raise error
        match &self {
            UInt::UInt8(num) => UInt::validate_length(&[*num], length),
            UInt::UInt16(num) => {
                let mut buf = [0u8; 2];
                u16_to_bytes(*num, &mut buf);
                UInt::validate_length(&buf, length)
            }
            UInt::UInt32(num) => {
                let mut buf = [0u8; 4];
                u32_to_bytes(*num, &mut buf);
                UInt::validate_length(&buf, length)
            }
            UInt::UInt64(num) => {
                let mut buf = [0u8; 8];
                u64_to_bytes(*num, &mut buf);
                UInt::validate_length(&buf, length)
            }
            UInt::UInt128(num) => {
                let mut buf = [0u8; 16];
                u128_to_bytes(*num, &mut buf);
                UInt::validate_length(&buf, length)
            }
            UInt::UIntFlex(array) => UInt::validate_length(&array, length),
        }
    }
}
