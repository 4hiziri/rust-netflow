#![allow(dead_code)]
use field::{NetFlowField, NetFlowOption, NetFlowScope};
use nom;
use nom::{be_u16, be_u32};

// FIXME: skip padding while parsing

// Netflow(1|5|9|..) -> flowset(Template|Option|Data)+

// parser
named!(netflow_version <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

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

named!(netflow9_count <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(netflow9_sys_uptime <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_timestamp <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_flow_sequence <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_source_id <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));

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
        let version = version.unwrap().1;

        if version == 9 {
            let (payload, count) = netflow9_count(payload).unwrap();
            let (payload, sys_uptime) = netflow9_sys_uptime(payload).unwrap();
            let (payload, timestamp) = netflow9_timestamp(payload).unwrap();
            let (payload, flow_sequence) = netflow9_flow_sequence(payload).unwrap();
            let (payload, source_id) = netflow9_source_id(payload).unwrap();
            let flow_sets = NetFlow9::parse_flowsets(payload).unwrap();

            Some(NetFlow9 {
                version: version,
                count: count.unwrap().1,
                sys_uptime: sys_uptime.unwrap().1,
                timestamp: timestamp.unwrap().1,
                flow_sequence: flow_sequence.unwrap().1,
                source_id: source_id.unwrap().1,
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

named!(flowset_id <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(flowset_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

const TEMPLATE_FLOWSET_ID: u16 = 0;
const OPTION_FLOWSET_ID: u16 = 1;

impl FlowSet {
    fn from_slice(data: &[u8]) -> Result<(&[u8], FlowSet), ()> {
        let (_, id) = flowset_id(&data).unwrap();

        match id.unwrap().1 {
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

// TODO: need mut?
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

named!(template_id <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(template_field_count <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(netflowfield <&[u8], NetFlowField>, dbg!(map!(count!(map!(take!(2), be_u16), 2),
                                                     |v: Vec<_>| NetFlowField::new(v[0].clone().unwrap().1, v[1].clone().unwrap().1))));

impl DataTemplate {
    pub fn new(
        length: u16,
        template_id: u16,
        field_count: u16,
        fields: Vec<NetFlowField>,
    ) -> DataTemplate {
        DataTemplate {
            flowset_id: 0, // DataTemplate's flowset_id is 0
            length: length,
            template_id: template_id,
            field_count: field_count,
            fields: fields,
        }
    }

    pub fn from_slice(data: &[u8]) -> Result<(&[u8], DataTemplate), ()> {
        // TODO: define Error type
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let flowset_id = flowset_id.unwrap().1;
        let (rest, flowset_length) = flowset_length(&rest).unwrap();
        let (rest, template_id) = template_id(&rest).unwrap();
        let (rest, template_field_count) = template_field_count(&rest).unwrap();
        let template_field_count = template_field_count.unwrap().1;
        let (rest, field_vec): (&[u8], Vec<NetFlowField>) =
            parse_netflowfield(template_field_count as usize, &rest).unwrap();


        if flowset_id == TEMPLATE_FLOWSET_ID {
            Ok((
                rest,
                DataTemplate {
                    flowset_id: flowset_id,
                    length: flowset_length.unwrap().1,
                    template_id: template_id.unwrap().1,
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
    scopes: Vec<NetFlowScope>,
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

named!(option_scope_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(option_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(netflowscope <&[u8], NetFlowScope>, map!(count!(map!(take!(2), be_u16), 2),
                                                  |v: Vec<_>| NetFlowScope::new(v[0].clone().unwrap().1, v[1].clone().unwrap().1)));
named!(netflowoption <&[u8], NetFlowOption>, map!(count!(map!(take!(2), be_u16), 2),
                                                  |v: Vec<_>| NetFlowOption::new(v[0].clone().unwrap().1, v[1].clone().unwrap().1)));

impl OptionTemplate {
    pub fn from_slice(data: &[u8]) -> Result<(&[u8], OptionTemplate), ()> {
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let flowset_id = flowset_id.unwrap().1;

        if flowset_id == OPTION_FLOWSET_ID {
            let (rest, length) = flowset_length(&rest).unwrap();
            let (rest, template_id) = template_id(&rest).unwrap();
            let (rest, scope_len) = option_scope_length(&rest).unwrap();
            let scope_len = scope_len.unwrap().1;
            let (rest, option_len) = option_length(&rest).unwrap();
            let option_len = option_len.unwrap().1;
            let mut scopes = Vec::<NetFlowScope>::with_capacity((scope_len / 4) as usize);
            let mut options = Vec::<NetFlowOption>::with_capacity((option_len / 4) as usize);

            let mut rest = rest;
            for _ in 0..(scope_len / 4) {
                let (next, scope) = netflowscope(rest).unwrap();
                scopes.push(scope);
                rest = next;
            }

            for _ in 0..(option_len / 4) {
                let (next, option) = netflowoption(rest).unwrap();
                options.push(option);
                rest = next;
            }

            Ok((
                rest,
                OptionTemplate {
                    flowset_id: flowset_id,
                    length: length.unwrap().1,
                    template_id: template_id.unwrap().1,
                    option_scope_length: scope_len,
                    option_length: option_len,
                    scopes: scopes,
                    options: options,
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
        let (rest, flowset_id) = flowset_id(&data).unwrap();

        Ok((
            rest,
            DataFlow {
                flowset_id: flowset_id.unwrap().1,
                length: 0,
                records: Vec::<u16>::new(), // TODO: parser
            },
        ))
    }
}

#[cfg(test)]
mod flowset_tests {
    use super::*;

    #[test]
    fn test_data_template() {
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

    #[test]
    fn test_option_template() {
        let packet_bytes = vec![
            0x00,
            0x01,
            0x00,
            0x1a,
            0x10,
            0x00,
            0x00,
            0x04,
            0x00,
            0x0c,
            0x00,
            0x01,
            0x00,
            0x04,
            0x00,
            0x30,
            0x00,
            0x01,
            0x00,
            0x31,
            0x00,
            0x01,
            0x00,
            0x32,
            0x00,
            0x04,
        ];

        let option: Result<(&[u8], OptionTemplate), ()> = OptionTemplate::from_slice(&packet_bytes);
        assert!(option.is_ok());

        let (_rest, option): (&[u8], OptionTemplate) = option.unwrap();
        assert_eq!(option.flowset_id, 1, "wrong OptionTemplate.flowset_id.");
        assert_eq!(option.length, 26, "wrong OptionTemplate.length.");
        assert_eq!(
            option.template_id,
            4096,
            "wrong OptionTemplate.template_id."
        );
        assert_eq!(
            option.option_scope_length,
            4,
            "wrong OptionTemplate.option_scope_length."
        );
        assert_eq!(
            option.option_length,
            12,
            "wrong Optiontemplate.option_length."
        );
    }
}
