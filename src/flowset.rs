#![allow(dead_code)]
use field::{TypeLengthField, FlowField};
use nom;
use nom::{be_u16, be_u32};

// FIXME: skip padding while parsing
// TODO: parser with template for DataFlow, use hashmap like object?
// TODO: impl method struct into bytes

// Netflow V9 -> Header + (Template* Option* Data*)

named!(netflow_version <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

// TODO: enum NetFlow or abstract with Netflow struct
#[derive(Debug)]
pub struct NetFlow9 {
    pub version: u16,
    pub count: u16,
    pub sys_uptime: u32, // FIXME: replace proper type like time
    pub timestamp: u32,
    pub flow_sequence: u32,
    pub source_id: u32,
    pub flow_sets: Vec<FlowSet>,
}

named!(netflow9_count <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(netflow9_sys_uptime <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_timestamp <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_flow_sequence <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_source_id <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));

impl NetFlow9 {
    fn parse_flowsets(data: &[u8]) -> Result<Vec<FlowSet>, ()> {
        let mut rest: &[u8] = data;
        let mut flowsets = Vec::<FlowSet>::new();

        while rest.len() != 0 {
            let (next, flowset) = FlowSet::from_bytes(&rest).unwrap();
            flowsets.push(flowset);
            rest = next;
        }

        Ok(flowsets)
    }

    pub fn from_bytes(payload: &[u8]) -> Result<NetFlow9, ()> {
        let (payload, version) = netflow_version(payload).unwrap();
        let version = version.unwrap().1;

        if version == 9 {
            let (payload, count) = netflow9_count(payload).unwrap();
            let (payload, sys_uptime) = netflow9_sys_uptime(payload).unwrap();
            let (payload, timestamp) = netflow9_timestamp(payload).unwrap();
            let (payload, flow_sequence) = netflow9_flow_sequence(payload).unwrap();
            let (payload, source_id) = netflow9_source_id(payload).unwrap();
            let flow_sets = NetFlow9::parse_flowsets(payload).unwrap();

            Ok(NetFlow9 {
                version: version,
                count: count.unwrap().1,
                sys_uptime: sys_uptime.unwrap().1,
                timestamp: timestamp.unwrap().1,
                flow_sequence: flow_sequence.unwrap().1,
                source_id: source_id.unwrap().1,
                flow_sets: flow_sets,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub enum FlowSet {
    DataTemplate(DataTemplate),
    OptionTemplate(OptionTemplate),
    DataFlow(DataFlow),
}

named!(flowset_id <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(flowset_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

const TEMPLATE_FLOWSET_ID: u16 = 0;
const OPTION_FLOWSET_ID: u16 = 1;

impl FlowSet {
    // TODO: parse with template
    fn from_bytes(data: &[u8]) -> Result<(&[u8], FlowSet), ()> {
        let (_, id) = flowset_id(&data).unwrap();
        let id = id.unwrap().1;
        info!("parsed flowset id: {:?}", id);

        match id {
            TEMPLATE_FLOWSET_ID => {
                let (next, template) = DataTemplate::from_bytes(&data).unwrap(); // TODO: use combinator
                debug!("parsed DataTemplate: {:?}", template);
                Ok((next, FlowSet::DataTemplate(template)))
            }
            OPTION_FLOWSET_ID => {
                let (next, option) = OptionTemplate::from_bytes(&data).unwrap();
                debug!("parsed OptionTemplate: {:?}", option);
                Ok((next, FlowSet::OptionTemplate(option)))
            }
            _ => {
                let (next, flow) = DataFlow::from_bytes_notemplate(&data).unwrap();
                debug!("parsed DataFlow: {:?}", flow);
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
    pub fields: Vec<TypeLengthField>,
}

named!(template_id <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(template_field_count <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

impl DataTemplate {
    pub fn new(
        length: u16,
        template_id: u16,
        field_count: u16,
        fields: Vec<TypeLengthField>,
    ) -> DataTemplate {
        DataTemplate {
            flowset_id: 0, // DataTemplate's flowset_id is 0
            length: length,
            template_id: template_id,
            field_count: field_count,
            fields: fields,
        }
    }

    fn validate_length(field_count: u16, length: u16) -> bool {
        2 + 2 + 2 + 2 + field_count * 4 == length
    }

    pub fn from_bytes(data: &[u8]) -> Result<(&[u8], DataTemplate), ()> {
        // TODO: define Error type
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let flowset_id = flowset_id.unwrap().1;
        let (rest, flowset_length) = flowset_length(&rest).unwrap();
        let flowset_length = flowset_length.unwrap().1;
        let (rest, template_id) = template_id(&rest).unwrap();
        let (rest, field_count) = template_field_count(&rest).unwrap();
        let field_count = field_count.unwrap().1;
        let (rest, field_vec): (&[u8], Vec<TypeLengthField>) =
            TypeLengthField::take_from(field_count as usize, &rest).unwrap();

        if !DataTemplate::validate_length(field_count, flowset_length) {
            debug!(
                "DataTemplate length is wrong. 8 + {} * 4 != {}",
                field_count,
                flowset_length
            );
        }

        if flowset_id == TEMPLATE_FLOWSET_ID {
            Ok((
                rest,
                DataTemplate::new(
                    flowset_length,
                    template_id.unwrap().1,
                    field_count,
                    field_vec,
                ),
            ))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct OptionTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub template_id: u16,
    pub option_scope_length: u16,
    pub option_length: u16,
    pub scopes: Vec<TypeLengthField>,
    pub options: Vec<TypeLengthField>,
}

named!(option_scope_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(option_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

impl OptionTemplate {
    pub fn from_bytes(data: &[u8]) -> Result<(&[u8], OptionTemplate), ()> {
        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let flowset_id = flowset_id.unwrap().1;

        if flowset_id == OPTION_FLOWSET_ID {
            let (rest, length) = flowset_length(&rest).unwrap();
            let (rest, template_id) = template_id(&rest).unwrap();
            let (rest, scope_len) = option_scope_length(&rest).unwrap();
            let scope_len = scope_len.unwrap().1;
            let (rest, option_len) = option_length(&rest).unwrap();
            let option_len = option_len.unwrap().1;
            let (rest, scopes) = TypeLengthField::take_from((scope_len / 4) as usize, rest)
                .unwrap();
            let (rest, options) = TypeLengthField::take_from((option_len / 4) as usize, rest)
                .unwrap();

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
    record_bytes: Option<Vec<u8>>,
    records: Option<Vec<FlowField>>,
}

impl DataFlow {
    // pub fn new<T>(flowset_id: u16, length: u16, records: Option<Vec<T>>) -> DataFlow {
    //     DataFlow {
    //         flowset_id: flowset_id,
    //         length: length,
    //         record_bytes: None,
    //         records: records,
    //     }
    // }

    pub fn from_bytes_notemplate(data: &[u8]) -> Result<(&[u8], DataFlow), ()> {
        // Some implementation can append padding, so I can't parse dataflow without template information.
        info!("WARNING: from_bytes_notemplate may not work! I can't handle padding correctly without Templates.")
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, length) = flowset_length(&rest).unwrap();
        let length = length.unwrap().1;
        let record_bytes = &rest[..(length as usize - 4)];
        let rest = &rest[(length as usize - 4)..];
        // TODO: need field parser for skipping padding

        Ok((
            rest,
            DataFlow {
                flowset_id: flowset_id.unwrap().1,
                length: length,
                record_bytes: Some(record_bytes.to_vec()),
                records: None,
            },
        ))
    }

    pub fn from_bytes<'a>(
        data: &'a [u8],
        templates: &[DataTemplate],
    ) -> Result<(&'a [u8], DataFlow), ()> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let flowset_id = flowset_id.unwrap().1;
        let (rest, length) = flowset_length(&rest).unwrap();
        let length = length.unwrap().1;

        // TODO: need field parser for skipping padding

        let template: Vec<&DataTemplate> = templates
            .iter()
            .filter(|temp| temp.template_id == flowset_id)
            .collect();

        if template.len() == 0 {
            debug!("invalid template, found {}", template.len());
            Err(())
        } else {
            let template = template[0];
            let mut rest = rest;
            let mut fields: Vec<FlowField> = Vec::with_capacity(template.fields.len());

            for field in &template.fields {
                let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, rest)
                    .unwrap();

                fields.push(flow_field);
                rest = next;
            }

            Ok((
                rest,
                DataFlow {
                    flowset_id: flowset_id,
                    length: length,
                    record_bytes: None,
                    records: Some(fields),
                },
            ))
        }
    }

    // fn validate_length()
}
