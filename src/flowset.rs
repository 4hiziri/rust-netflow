#![allow(dead_code)]
use field::{NetFlowField, NetFlowOption};
use byteorder::{BigEndian, ReadBytesExt};

// Netflow(1|5|9|..) -> flowset(Template|Option|Data)+

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
// TODO: impl flowset parsers later
// TODO: enum NetFlow
// TODO: abstract with Netflow struct
#[derive(Debug)]
pub struct NetFlow9 {
    version: u16,
    count: u16,
    sys_uptime: u32,
    timestamp: u32,
    flow_sequence: u32,
    source_id: u32,
    flow_sets: Vec<u8>,
}

named!(netflow9_count <&[u8], u16>, map!(take!(2), to_u16));
named!(netflow9_sys_uptime <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_timestamp <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_flow_sequence <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_source_id <&[u8], u32>, map!(take!(4), to_u32));

// TODO: use nom to parse payload?
impl NetFlow9 {
    pub fn new(payload: &[u8]) -> Option<NetFlow9> {
        let (payload, version) = netflow_version(payload).unwrap();
        if version == 9 {
            let (payload, count) = netflow9_count(payload).unwrap();
            let (payload, sys_uptime) = netflow9_sys_uptime(payload).unwrap();
            let (payload, timestamp) = netflow9_timestamp(payload).unwrap();
            let (payload, flow_sequence) = netflow9_flow_sequence(payload).unwrap();
            let (payload, source_id) = netflow9_source_id(payload).unwrap();
            let flow_sets = payload; // parse?

            Some(NetFlow9 {
                version: version,
                count: count,
                sys_uptime: sys_uptime,
                timestamp: timestamp,
                flow_sequence: flow_sequence,
                source_id: source_id,
                flow_sets: flow_sets.to_vec(),
            })
        } else {
            None
        }
    }
}

enum FlowSet {
    DataTemplate(DataTemplate),
    OptionTemplate(OptionTemplate),
    DataFlow(DataFlow),
}

named!(flowset_id <&[u8], u16>, map!(take!(2), to_u16));
named!(flowset_length <&[u8], u16>, map!(take!(2), to_u16));

const TEMPLATE_FLOWSET_ID: u16 = 0;
const OPTION_FLOWSET_ID: u16 = 1;

impl FlowSet {
    fn parse_flow(data: &[u8]) -> FlowSet {
        let (_, id) = flowset_id(&data).unwrap();
        // let (rest, length) = flowset_length(&rest).unwrap();

        match id {
            TEMPLATE_FLOWSET_ID => FlowSet::DataTemplate(DataTemplate::new(&data).unwrap()),
            OPTION_FLOWSET_ID => FlowSet::OptionTemplate(OptionTemplate::new(&data).unwrap()),
            _ => FlowSet::DataFlow(DataFlow::new(&data).unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataTemplate {
    flowset_id: u16,
    length: u16,
    template_id: u16,
    field_count: u16,
    fields: Vec<NetFlowField>,
}

fn parse_netflowfield(data: &[u8]) -> NetFlowField {
    NetFlowField::new(to_u16(&data[0..1]), to_u16(&data[2..3]))
}

named!(template_id <&[u8], u16>, map!(take!(2), to_u16));
named!(template_field_count <&[u8], u16>, map!(take!(2), to_u16));
named!(netflowfield <&[u8], NetFlowField>, map!(take!(4), parse_netflowfield)); // TODO: extract parsers?

impl DataTemplate {
    pub fn new(data: &[u8]) -> Option<DataTemplate> {
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, flowset_length) = flowset_length(&rest).unwrap();
        let (rest, template_id) = template_id(&rest).unwrap();
        let (_rest, template_field_count) = template_field_count(&rest).unwrap();

        if flowset_id == TEMPLATE_FLOWSET_ID {
            Some(DataTemplate {
                flowset_id: flowset_id,
                length: flowset_length,
                template_id: template_id,
                field_count: template_field_count,
                fields: Vec::<NetFlowField>::new(), // TODO: Add parser impl
            })
        } else {
            None
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

impl OptionTemplate {
    pub fn new(data: &[u8]) -> Option<OptionTemplate> {
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, length) = flowset_length(&rest).unwrap();
        let (rest, template_id) = template_id(&rest).unwrap();

        if flowset_id == OPTION_FLOWSET_ID {
            Some(OptionTemplate {
                flowset_id: flowset_id,
                length: length,
                template_id: 0,
                option_scope_length: 0,
                option_length: 0,
                options: Vec::<NetFlowOption>::new(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataFlow {
    flowset_id: u16,
    length: u16,
    records: Vec<u16>,
}

impl DataFlow {
    pub fn new(data: &[u8]) -> Option<DataFlow> {
        let (rest, flowset_id) = flowset_id(&data).unwrap();

        DataFlow {
            flowset_id: flowset_id,
            length: 0,
            records: Vec::<u16>::new(), // TODO: parser
        };

        None
    }
}
