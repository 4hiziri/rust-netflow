use nom::{be_u16, be_u32, be_u64};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::collections::HashSet;
use std::convert::From;

pub fn be_u128(i: &[u8]) -> Result<(&[u8], u128), ()> {
    if i.len() < 16 {
        Err(())
    } else {
        let res = ((i[0] as u128) << 120) + ((i[1] as u128) << 112) + ((i[2] as u128) << 104) +
            ((i[3] as u128) << 96) +
            ((i[4] as u128) << 88) + ((i[5] as u128) << 80) +
            ((i[6] as u128) << 72) + ((i[7] as u128) << 64) +
            ((i[8] as u128) << 56) +
            ((i[9] as u128) << 48) + ((i[10] as u128) << 40) +
            ((i[11] as u128) << 32) +
            ((i[12] as u128) << 24) +
            ((i[13] as u128) << 16) + ((i[14] as u128) << 8) + (i[15] as u128);
        Ok((&i[16..], res))
    }
}

named!(netflowfield <&[u8], TypeLengthField>,
       dbg!(map!(count!(map!(take!(2), be_u16), 2),
                 |v: Vec<_>| TypeLengthField::new(v[0].clone().unwrap().1, v[1].clone().unwrap().1))));

#[derive(Debug, Clone, Copy)]
pub struct TypeLengthField {
    pub type_id: u16,
    pub length: u16,
}

impl TypeLengthField {
    pub fn new(type_id: u16, length: u16) -> TypeLengthField {
        TypeLengthField {
            type_id: type_id,
            length: length,
        }
    }

    pub fn take_from(count: usize, data: &[u8]) -> Result<(&[u8], Vec<TypeLengthField>), ()> {
        // TODO: define Error type
        let mut rest = data;
        let mut field_vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (next, field) = netflowfield(&rest).unwrap();
            field_vec.push(field);
            rest = next;
        }

        Ok((rest, field_vec))
    }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod FieldTypes {
    pub const IN_BYTES: u16 = 1; // N, default is 4
    pub const IN_PKTS: u16 = 2; // N, default is 4
    pub const FLOWS: u16 = 3; // N, default is 4
    pub const PROTOCOL: u16 = 4; // 1
    pub const TOS: u16 = 5; // 1
    pub const TCP_FLAGS: u16 = 6; // 1
    pub const L4_SRC_PORT: u16 = 7; // 2
    pub const IPV4_SRC_ADDR: u16 = 8; // 4, IPv4 address
    pub const SRC_MASK: u16 = 9; // 1,
    pub const INPUT_SNMP: u16 = 10; // N, default is 2
    pub const IPV4_DST_ADDR: u16 = 12; // 4, IPv4 address
    pub const DST_MASK: u16 = 13; // 1
    pub const OUTPUT_SNMP: u16 = 14; // N, default is 2
    pub const IPV4_NEXT_HOP: u16 = 15; // 4, IPv4 address
    pub const SRC_AS: u16 = 16; // N, default is 2, [2, 4]
    pub const DST_AS: u16 = 17; // N, default is 2, [2, 4]
    pub const BGP_IPV4_NEXT_HOP: u16 = 18; // 4, IPv4 address
    pub const MUL_DST_PKTS: u16 = 19; // N, default is 4
    pub const MUL_DST_BYTES: u16 = 20; // N, default is 4
    pub const LAST_SWITCHED: u16 = 21; // 4
    pub const FIRST_SWITCHED: u16 = 22; // 4
    pub const OUT_BYTES: u16 = 23; // N, default is 4
    pub const OUT_PKTS: u16 = 24; // N, default is 4
    pub const MIN_PKT_LENGTH: u16 = 25; // 2
    pub const MAX_PKT_LENGTH: u16 = 26; // 2
    pub const IPV6_SRC_ADDR: u16 = 27; // 16, IPv6 address
    pub const IPV6_DST_ADDR: u16 = 28; // 16, IPv6 address
    pub const IPV6_SRC_MASK: u16 = 29; // 1
    pub const IPV6_DST_MASK: u16 = 30; // 1
    pub const IPV6_FLOW_LABEL: u16 = 31; // 3
    pub const ICMP_TYPE: u16 = 32; //2, ICMP_Type * 256 + ICMP_Code
    pub const MUL_IGMP_TYPE: u16 = 33; // 1
    pub const SAMPLING_INTERVAL: u16 = 34; // 4
    //0x01 Deterministic Sampling ,0x02 Random Sampling
    pub const SAMPLING_ALGORITHM: u16 = 35; // 1
    pub const FLOW_ACTIVE_TIMEOUT: u16 = 36; // 2
    pub const FLOW_INACTIVE_TIMEOUT: u16 = 37; // 2
    pub const ENGINE_TYPE: u16 = 38; // 1
    pub const ENGINE_ID: u16 = 39; // 1
    pub const TOTAL_BYTES_EXP: u16 = 40; // N, default is 4
    pub const TOTAL_PKTS_EXP: u16 = 41; // N, default is 4
    pub const TOTAL_FLOWS_EXP: u16 = 42; // N, default is 4
    pub const VENDOR_PROPRIETARY_43: u16 = 43; // N
    pub const IPV4_SRC_PREFIX: u16 = 44; // 4, IPv4 address, specific cisco
    pub const IPV4_DST_PREFIX: u16 = 45; // 4, IPv4 address, specific cisco
    // 0x00 UNKNOWN 0x01 TE-MIDPT 0x02 ATOM 0x03 VPN 0x04 BGP 0x05 LDP
    pub const MPLS_TOP_LABEL_TYPE: u16 = 46; // 1
    pub const MPLS_TOP_LABEL_IP_ADDR: u16 = 47; // 4
    pub const FLOW_SAMPLER_ID: u16 = 48; // 1
    pub const FLOW_SAMPLER_MODE: u16 = 49; // 1
    pub const FLOW_SAMPLER_RANDOM_INTERVAL: u16 = 50; // 4
    pub const VENDOR_PROPRIETARY_51: u16 = 51; // N
    pub const MIN_TTL: u16 = 52; // 1
    pub const MAX_TTL: u16 = 53; // 1
    pub const IPV4_IDENT: u16 = 54; // 2
    pub const DST_TOS: u16 = 55; // 1
    pub const SRC_MAC: u16 = 56; // 6, MAC address
    pub const DST_MAC: u16 = 57; // 6, MAC address
    pub const SRC_VLAN: u16 = 58; // 2
    pub const DST_VLAN: u16 = 59; // 2
    // if there isn't this field, v4 is assumed
    pub const IP_PROTOCOL_VERSION: u16 = 60; // 1
    // 0 = ingres flow, 1 = egress flow
    pub const DIRECTION: u16 = 61; // 1
    pub const IPV6_NEXT_HOP: u16 = 62; // 16, IPv6 address
    pub const BGP_IPV6_NEXT_HOP: u16 = 63; // 16, IPv6 address
    pub const IPV6_OPTION_HEADERS: u16 = 64; // 4
    pub const VENDOR_PROPRIETARY_65: u16 = 65; // N
    pub const VENDOR_PROPRIETARY_66: u16 = 66; // N
    pub const VENDOR_PROPRIETARY_67: u16 = 67; // N
    pub const VENDOR_PROPRIETARY_68: u16 = 68; // N
    pub const VENDOR_PROPRIETARY_69: u16 = 69; // N
    pub const MPLS_LABEL_1: u16 = 70; // 3
    pub const MPLS_LABEL_2: u16 = 71; // 3
    pub const MPLS_LABEL_3: u16 = 72; // 3
    pub const MPLS_LABEL_4: u16 = 73; // 3
    pub const MPLS_LABEL_5: u16 = 74; // 3
    pub const MPLS_LABEL_6: u16 = 75; // 3
    pub const MPLS_LABEL_7: u16 = 76; // 3
    pub const MPLS_LABEL_8: u16 = 77; // 3
    pub const MPLS_LABEL_9: u16 = 78; // 3
    pub const MPLS_LABEL_10: u16 = 79; // 3
    pub const IN_DST_MAC: u16 = 80; // 6, MAC address
    pub const OUT_SRC_MAC: u16 = 81; // 6, MAC address
    pub const IF_NAME: u16 = 82; // N, string
    pub const IF_DESC: u16 = 83; // N, string
    pub const SAMPLER_NAME: u16 = 84; // N, string
    pub const IN_PERMANENT_BYTES: u16 = 85; // N, default is 4
    pub const IN_PERMANENT_PKTS: u16 = 86; // N, default is 4
    pub const VENDOR_PROPRIETARY_87: u16 = 87; // N
    pub const FRAGMENT_OFFSET: u16 = 88; // 2
    // check bit meanings
    pub const FORWARDING_STATUS: u16 = 89; // 1
    pub const MPLS_PAL_RD: u16 = 90; // 8, array
    pub const MPLS_PREFIX_LEN: u16 = 91; // 1
    pub const SRC_TRAFFIC_INDEX: u16 = 92; // 4
    pub const DST_TRAFFIC_INDEX: u16 = 93; // 4
    pub const APPLICATION_DESCRIPTION: u16 = 94; // N
    pub const APPLICATION_TAG: u16 = 95; //1 + n, n is bit
    pub const APPLICATION_NAME: u16 = 96; // N
    pub const postipDiffServCodePoint: u16 = 98; // 1
    pub const replication_factor: u16 = 99; // 4
    pub const DEPRECATED: u16 = 100; // N
    pub const layer2packetSectionOffset: u16 = 102; // N
    pub const layer2packetSectionSize: u16 = 103; // N
    pub const layer2packetSectionData: u16 = 104; // N
    // 105 to 127 are reserved
    // 128 to 32768 are in IANA
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ScopeTypes {
    pub const System: u16 = 1;
    pub const Interface: u16 = 2;
    pub const Line_Card: u16 = 3;
    pub const NetFlow_Cache: u16 = 4;
    pub const Template: u16 = 5;
}

// research types
// 1. flexible length num, length = N bytes
// 2. fixed length num, length = 1, 2, 3, 4
// 3. byte array
// 4. ipv4 address
// 5. ipv6 address
// 6. mac address
// 7. string

#[derive(Debug, Clone)]
pub enum FieldValue {
    NumField(UInt),
    ByteArray(Vec<u8>),
    Ipv4Addr(Ipv4Addr),
    Ipv6Addr(Ipv6Addr),
    MacAddr(MacAddr),
    String(String),
    Unknown(Vec<u8>),
}

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
            let ip = be_u32(&value).unwrap().1;
            FieldValue::Ipv4Addr(Ipv4Addr::from(ip))
        } else if FieldValue::is_ipv6_field(type_id) {
            FieldValue::Ipv6Addr(Ipv6Addr::from(be_u128(value).unwrap().1))
        } else if FieldValue::is_mac_field(type_id) {
            FieldValue::MacAddr(MacAddr::new(
                value[0],
                value[1],
                value[2],
                value[3],
                value[4],
                value[5],
            ))
        } else if FieldValue::is_string_field(type_id) {
            FieldValue::String(String::from_utf8(value.to_vec()).unwrap())
        } else if FieldValue::is_bits_field(type_id) {
            FieldValue::NumField(UInt::from_bytes(value))
        } else {
            FieldValue::ByteArray(value.to_vec())
        }
    }
}

#[derive(Debug, Clone)]
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
    // convert uint from [u8] as BigEndian
    pub fn from_bytes(bytes: &[u8]) -> UInt {
        let len = bytes.len();

        if len == 1 {
            UInt::UInt8(bytes[0])
        } else if len == 2 {
            UInt::UInt16(be_u16(&bytes).unwrap().1)
        } else if len > 2 && len <= 4 {
            UInt::UInt32(be_u32(&bytes).unwrap().1)
        } else if len > 4 && len <= 8 {
            UInt::UInt64(be_u64(&bytes).unwrap().1)
        } else if len > 8 && len <= 16 {
            UInt::UInt128(be_u128(&bytes).unwrap().1)
        } else {
            UInt::UIntFlex(bytes.to_vec())
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlowField {
    type_id: u16,
    length: u16,
    value: FieldValue,
}

impl FlowField {
    pub fn new(type_id: u16, length: u16, value: FieldValue) -> FlowField {
        FlowField {
            type_id: type_id,
            length: length,
            value: value,
        }
    }

    pub fn from_bytes(type_id: u16, length: u16, bytes: &[u8]) -> Result<(&[u8], FlowField), ()> {
        if (length as usize) >= bytes.len() {
            Ok((
                &bytes[(length as usize)..],
                FlowField::new(
                    type_id,
                    length,
                    FieldValue::new(type_id, &bytes[..(length as usize)]),
                ),
            ))
        } else {
            Err(())
        }
    }
}

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
}
