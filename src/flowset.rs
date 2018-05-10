#![allow(dead_code)]

use field::{NetFlowField, NetFlowOption};
use nom;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct DataTemplate {
    flowset_id: u16,
    length: u16,
    template_id: u16,
    field_count: u16,
    fields: Vec<NetFlowField>,
}

impl DataTemplate {
    pub fn new() -> DataTemplate {
        DataTemplate {
            flowset_id: 0,
            length: 0,
            template_id: 0,
            field_count: 0,
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionTemplate {
    flowset_id: u16,
    length: u16,
    template_id: u16,
    option_scope_length: u16,
    option_length: u16,
    options: Vec<NetFlowOption>,
}

#[derive(Debug, Clone)]
pub struct DataFlow {
    flowset_id: u16,
    length: u16,
    records: Vec<u16>,
}

// TODO: need?
pub trait FlowSet {}

fn to_u16(bytes: &[u8]) -> u16 {
    let mut buf = &bytes[..];
    buf.read_u16::<BigEndian>().unwrap()
}

fn to_u32(bytes: &[u8]) -> u32 {
    let mut buf = &bytes[..];
    buf.read_u32::<BigEndian>().unwrap()
}

// parser
named!(netflow_version <&[u8], u16>, map!(take!(2), to_u16));
named!(netflow9_count <&[u8], u16>, map!(take!(2), to_u16));
named!(netflow9_sys_uptime <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_timestamp <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_flow_sequence <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_flowset_id <&[u8], u32>, map!(take!(4), to_u32));
// TODO: impl flowset parsers later

pub fn get_version(payload: &[u8]) -> u16 {
    (payload[0] as u16) << 8 + payload[1] as u16
}

// TODO: abstract with Netflow struct
#[derive(Debug)]
pub struct NetFlow9 {
    version: u16,
    count: u16,
    sys_uptime: u32,
    timestamp: u32,
    flow_sequence: u32,
    flowset_id: u32,
    flow_sets: Vec<u8>,
}

// TODO: use nom to parse payload?
impl NetFlow9 {
    pub fn new(payload: &[u8]) -> Option<NetFlow9> {
        let (payload, version) = netflow_version(payload).unwrap();
        if version == 9 {
            let (payload, count) = netflow9_count(payload).unwrap();
            debug!("payload after count: {:?}", payload);
            let (payload, sys_uptime) = netflow9_sys_uptime(payload).unwrap();
            debug!("payload after sys_uptime: {:?}", payload);
            let (payload, timestamp) = netflow9_timestamp(payload).unwrap();
            let (payload, flow_sequence) = netflow9_flow_sequence(payload).unwrap();
            let (payload, flowset_id) = netflow9_flowset_id(payload).unwrap();
            let flow_sets = payload;

            Some(NetFlow9 {
                version: version,
                count: count,
                sys_uptime: sys_uptime,
                timestamp: timestamp,
                flow_sequence: flow_sequence,
                flowset_id: flowset_id,
                flow_sets: flow_sets.to_vec(),
            })
        } else {
            None
        }
    }
}
