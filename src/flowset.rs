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
    flow_sets: Vec<FlowSet>,
}

named!(netflow9_count <&[u8], u16>, map!(take!(2), to_u16));
named!(netflow9_sys_uptime <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_timestamp <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_flow_sequence <&[u8], u32>, map!(take!(4), to_u32));
named!(netflow9_source_id <&[u8], u32>, map!(take!(4), to_u32));

// TODO: use nom to parse payload?
impl NetFlow9 {
    fn parse_flowsets(data: &[u8]) -> Result<Vec<FlowSet>, ()> {
        let mut rest: &[u8] = data;
        let mut flowsets = Vec::<FlowSet>::new();

        while rest.len() != 0 {
            let (next, flowset) = FlowSet::from_slice(&rest).unwrap();
            flowsets.push(flowset);
            rest = next;
        }

        Ok(flowsets)
    }

    pub fn new(payload: &[u8]) -> Option<NetFlow9> {
        let (payload, version) = netflow_version(payload).unwrap();
        if version == 9 {
            let (payload, count) = netflow9_count(payload).unwrap();
            let (payload, sys_uptime) = netflow9_sys_uptime(payload).unwrap();
            let (payload, timestamp) = netflow9_timestamp(payload).unwrap();
            let (payload, flow_sequence) = netflow9_flow_sequence(payload).unwrap();
            let (payload, source_id) = netflow9_source_id(payload).unwrap();
            let flow_sets = NetFlow9::parse_flowsets(payload).unwrap();

            Some(NetFlow9 {
                version: version,
                count: count,
                sys_uptime: sys_uptime,
                timestamp: timestamp,
                flow_sequence: flow_sequence,
                source_id: source_id,
                flow_sets: flow_sets,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
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
    fn from_slice(data: &[u8]) -> Result<(&[u8], FlowSet), ()> {
        let (_, id) = flowset_id(&data).unwrap();

        match id {
            TEMPLATE_FLOWSET_ID => {
                let (next, template) = DataTemplate::from_slice(&data).unwrap(); // TODO: use combinator
                Ok((next, FlowSet::DataTemplate(template)))
            }
            OPTION_FLOWSET_ID => {
                let (next, option) = OptionTemplate::from_slice(&data).unwrap();
                Ok((next, FlowSet::OptionTemplate(option)))
            }
            _ => {
                let (next, flow) = DataFlow::from_slice(&data).unwrap();
                Ok((next, FlowSet::DataFlow(flow)))
            }
        }
    }
}

#[derive(Debug)]
pub struct DataTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub template_id: u16,
    pub field_count: u16,
    pub fields: Vec<NetFlowField>,
}

fn parse_netflowfield(count: usize, data: &[u8]) -> Result<(&[u8], Vec<NetFlowField>), ()> {
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

named!(template_id <&[u8], u16>, map!(take!(2), to_u16));
named!(template_field_count <&[u8], u16>, map!(take!(2), to_u16));
named!(netflowfield <&[u8], NetFlowField>, map!(count!(map!(take!(2), to_u16), 2),
                                                |v: Vec<u16>| NetFlowField::new(v[0], v[1])));

impl DataTemplate {
    pub fn from_slice(data: &[u8]) -> Result<(&[u8], DataTemplate), ()> {
        // TODO: define Error type
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, flowset_length) = flowset_length(&rest).unwrap();
        let (rest, template_id) = template_id(&rest).unwrap();
        let (rest, template_field_count) = template_field_count(&rest).unwrap();
        let (rest, field_vec): (&[u8], Vec<NetFlowField>) =
            parse_netflowfield(template_field_count as usize, &rest).unwrap();


        if flowset_id == TEMPLATE_FLOWSET_ID {
            Ok((
                rest,
                DataTemplate {
                    flowset_id: flowset_id,
                    length: flowset_length,
                    template_id: template_id,
                    field_count: template_field_count,
                    fields: field_vec,
                },
            ))
        } else {
            Err(())
        }
    }
}

// FIXME:
#[derive(Debug)]
pub struct OptionTemplate {
    flowset_id: u16,
    length: u16,
    template_id: u16,
    option_scope_length: u16,
    option_length: u16,
    options: Vec<NetFlowOption>,
}

fn parse_netflowoption(count: usize, data: &[u8]) -> Result<(&[u8], Vec<NetFlowOption>), ()> {
    // TODO: define Error type
    let mut rest = data;
    let mut field_vec = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let (next, option) = netflowoption(&rest).unwrap();
        field_vec.push(option);
        rest = next;
    }

    Ok((rest, field_vec))
}

named!(option_scope_length <&[u8], u16>, map!(take!(2), to_u16));
named!(option_length <&[u8], u16>, map!(take!(2), to_u16));
named!(netflowoption <&[u8], NetFlowOption>, map!(count!(map!(take!(2), to_u16), 2),
                                                |v: Vec<u16>| NetFlowOption::new(v[0], v[1])));

impl OptionTemplate {
    pub fn from_slice(data: &[u8]) -> Result<(&[u8], OptionTemplate), ()> {
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, length) = flowset_length(&rest).unwrap();
        let (rest, template_id) = template_id(&rest).unwrap();

        if flowset_id == OPTION_FLOWSET_ID {
            Ok((
                rest,
                OptionTemplate {
                    flowset_id: flowset_id,
                    length: length,
                    template_id: template_id,
                    option_scope_length: 0,
                    option_length: 0,
                    options: Vec::<NetFlowOption>::new(),
                },
            ))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct DataFlow {
    flowset_id: u16,
    length: u16,
    records: Vec<u16>,
}

impl DataFlow {
    pub fn from_slice(data: &[u8]) -> Result<(&[u8], DataFlow), ()> {
        let (_rest, flowset_id) = flowset_id(&data).unwrap();

        (DataFlow {
             flowset_id: flowset_id,
             length: 0,
             records: Vec::<u16>::new(), // TODO: parser
         });

        Err(())
    }
}

#[cfg(test)]
mod flowset_tests {
    use super::*;
    use env_logger;

    #[test]
    fn test_datatemplate() {
        let _logger = env_logger::init();
        let data_template_payload: Vec<u8> = vec![
            0x00,
            0x00,
            0x00,
            0x5c,
            0x04,
            0x00,
            0x00,
            0x15,
            0x00,
            0x15,
            0x00,
            0x04,
            0x00,
            0x16,
            0x00,
            0x04,
            0x00,
            0x01,
            0x00,
            0x04,
            0x00,
            0x02,
            0x00,
            0x04,
            0x00,
            0x3c,
            0x00,
            0x01,
            0x00,
            0x0a,
            0x00,
            0x02,
            0x00,
            0x0e,
            0x00,
            0x02,
            0x00,
            0x3d,
            0x00,
            0x01,
            0x00,
            0x03,
            0x00,
            0x04,
            0x00,
            0x08,
            0x00,
            0x04,
            0x00,
            0x0c,
            0x00,
            0x04,
            0x00,
            0x07,
            0x00,
            0x02,
            0x00,
            0x0b,
            0x00,
            0x02,
            0x00,
            0x05,
            0x00,
            0x01,
            0x00,
            0x06,
            0x00,
            0x01,
            0x00,
            0x04,
            0x00,
            0x01,
            0x00,
            0x38,
            0x00,
            0x06,
            0x00,
            0x50,
            0x00,
            0x06,
            0x00,
            0x3a,
            0x00,
            0x02,
            0x00,
            0xc9,
            0x00,
            0x04,
            0x00,
            0x30,
            0x00,
            0x01,
        ];

        // parsing process test
        let template: Result<(&[u8], DataTemplate), ()> =
            DataTemplate::from_slice(&data_template_payload);
        assert!(template.is_ok(), "DataTemplate failed to parse");

        // parsing result test
        let (_rest, template): (&[u8], DataTemplate) = template.unwrap();
        assert_eq!(template.flowset_id, 0, "DataTemplate has wrong flowset_id.");
        assert_eq!(template.length, 92, "DataTemplate has wrong length.");
        assert_eq!(
            template.template_id,
            1024,
            "Datatemplate has wrong template_id."
        );
        assert_eq!(
            template.field_count,
            21,
            "Datatemplate has wrong field_count."
        );
        // TODO: Field test
    }
}
