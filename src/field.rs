// TODO: impl converter for Field

use nom::be_u16;

named!(netflowfield <&[u8], NetFlowField>,
       dbg!(map!(count!(map!(take!(2), be_u16), 2),
                 |v: Vec<_>| NetFlowField::new(v[0].clone().unwrap().1, v[1].clone().unwrap().1))));

// TODO: from_str and compare(?)
#[derive(Debug, Clone, Copy)]
pub struct TypeLengthField {
    pub type_val: u16,
    pub length: u16,
}

impl TypeLengthField {
    pub fn new(type_val: u16, length: u16) -> TypeLengthField {
        TypeLengthField {
            type_val: type_val,
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
    pub const IPV6_FLOW_LABEL: u16 = 31; // 3, RFC2460
    pub const ICMP_TYPE: u16 = 32; //2, ICMP_Type * 256 + ICMP_Code
    pub const MUL_IGMP_TYPE: u16 = 33; // 1
    pub const SAMPLING_INTERVAL: u16 = 34; // 4
    pub const SAMPLING_ALGORITHM: u16 = 35; // 1, 0x01 Deterministic Sampling ,0x02 Random Sampling
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
    pub const MPLS_TOP_LABEL_TYPE: u16 = 46; // 1, 0x00 UNKNOWN 0x01 TE-MIDPT 0x02 ATOM 0x03 VPN 0x04 BGP 0x05 LDP
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
    pub const IP_PROTOCOL_VERSION: u16 = 60; // 1, if there isn't this field, v4 is assumed
    pub const DIRECTION: u16 = 61; // 1, 0 = ingres flow, 1 = egress flow
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
    pub const IN_PERMANENT_BYTES: u16 = 86; // N, default is 4
    pub const VENDOR_PROPRIETARY_87: u16 = 87; // N
    pub const FRAGMENT_OFFSET: u16 = 88; // 2
    pub const FORWARDING_STATUS: u16 = 89; // 1, check bit meanings
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
    pub const layer2packetSectionOffset: u16 = 102; //
    pub const layer2packetSectionSize: u16 = 103; //
    pub const layer2packetSectionData: u16 = 104; //
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

// FIXME: all template use TypeLengthField, and make NetFlowFields real field
pub type NetFlowField = TypeLengthField; // Field of DataTemplate
pub type NetFlowOption = TypeLengthField; // Field of OptionTemplate
pub type NetFlowScope = TypeLengthField; // Field of OptionScope

// TODO: impl each types of field, enum
pub struct Field {
    id: u16,
    value: Vec<u8>,
}

// research types
// 1. flexible length num, length = N bytes
// 2. fixed length num, length = 1, 2, 3, 4
// 3. byte array
// 4. ipv4 address
// 5. ipv6 address
// 6. mac address
// 7. string
