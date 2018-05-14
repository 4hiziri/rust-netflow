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
    pub const IN_BYTES: u16 = 1;
    pub const IN_PKTS: u16 = 2;
    pub const FLOWS: u16 = 3;
    pub const PROTOCOL: u16 = 4;
    pub const TOS: u16 = 5;
    pub const TCP_FLAGS: u16 = 6;
    pub const IPV4_SRC_ADDR: u16 = 8;
    pub const SRC_MASK: u16 = 9;
    pub const INPUT_SNMP: u16 = 10;
    pub const IPV4_DST_ADDR: u16 = 12;
    pub const DST_MASK: u16 = 13;
    pub const OUTPUT_SNMP: u16 = 14;
    pub const IPV4_NEXT_HOP: u16 = 15;
    pub const SRC_AS: u16 = 16;
    pub const DST_AS: u16 = 17;
    pub const BGP_IPV4_NEXT_HOP: u16 = 18;
    pub const MUL_DST_PKTS: u16 = 19;
    pub const MUL_DST_BYTES: u16 = 20;
    pub const LAST_SWITCHED: u16 = 21;
    pub const FIRST_SWITCHED: u16 = 22;
    pub const OUT_BYTES: u16 = 23;
    pub const OUT_PKTS: u16 = 24;
    pub const IPV6_SRC_ADDR: u16 = 27;
    pub const IPV6_DST_ADDR: u16 = 28;
    pub const IPV6_SRC_MASK: u16 = 29;
    pub const IPV6_DST_MASK: u16 = 30;
    pub const IPV6_FLOW_LABEL: u16 = 31;
    pub const ICMP_TYPE: u16 = 32;
    pub const MUL_IGMP_TYPE: u16 = 33;
    pub const SAMPLING_INTERVAL: u16 = 34;
    pub const SAMPLING_ALGORITHM: u16 = 35;
    pub const FLOW_ACTIVE_TIMEOUT: u16 = 36;
    pub const FLOW_INACTIVE_TIMEOUT: u16 = 37;
    pub const ENGINE_TYPE: u16 = 38;
    pub const ENGINE_ID: u16 = 39;
    pub const TOTAL_BYTES_EXP: u16 = 40;
    pub const TOTAL_PKTS_EXP: u16 = 41;
    pub const TOTAL_FLOWS_EXP: u16 = 42;
    pub const MPLS_TOP_LABEL_TYPE: u16 = 46;
    pub const MPLS_TOP_LABEL_IP_ADDR: u16 = 47;
    pub const FLOW_SAMPLER_ID: u16 = 48;
    pub const FLOW_SAMPLER_MODE: u16 = 49;
    pub const FLOW_SAMPLER_RANDOM_INTERVAL: u16 = 50;
    pub const DST_TOS: u16 = 55;
    pub const SRC_MAC: u16 = 56;
    pub const DST_MAC: u16 = 57;
    pub const SRC_VLAN: u16 = 58;
    pub const DST_VLAN: u16 = 59;
    pub const IP_PROTOCOL_VERSION: u16 = 60;
    pub const DIRECTION: u16 = 61;
    pub const IPV6_NEXT_HOP: u16 = 62;
    pub const BGP_IPV6_NEXT_HOP: u16 = 63;
    pub const IPV6_OPTION_HEADERS: u16 = 64;
    pub const MPLS_LABEL_1: u16 = 70;
    pub const MPLS_LABEL_2: u16 = 71;
    pub const MPLS_LABEL_3: u16 = 72;
    pub const MPLS_LABEL_4: u16 = 73;
    pub const MPLS_LABEL_5: u16 = 74;
    pub const MPLS_LABEL_6: u16 = 75;
    pub const MPLS_LABEL_7: u16 = 76;
    pub const MPLS_LABEL_8: u16 = 77;
    pub const MPLS_LABEL_9: u16 = 78;
    pub const MPLS_LABEL_10: u16 = 79;
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

pub struct Field {
    id: u16,
    value: Vec<u8>,
    is_byte_arrray: bool,
}
