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
        let id = id.unwrap().1;
        debug!("parsed flowset id: {}", id);

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
    pub flowset_id: u16,
    pub length: u16,
    pub template_id: u16,
    pub option_scope_length: u16,
    pub option_length: u16,
    pub scopes: Vec<NetFlowScope>,
    pub options: Vec<NetFlowOption>,
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
