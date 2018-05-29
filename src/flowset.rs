#![allow(dead_code)]
use field::{TypeLengthField, FlowField};
use nom;
use nom::{be_u16, be_u32};
use netflow::*;

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
    pub fn from_bytes(data: &[u8]) -> Result<(&[u8], FlowSet), ()> {
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

    // FIXME: to methode?
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

    pub fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> Result<(&'a [u8], Vec<FlowField>), ()> {
        let mut rest = payload;
        let mut fields: Vec<FlowField> = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, rest)
                .unwrap();

            fields.push(flow_field);
            rest = next;
        }

        Ok((&rest, fields))
    }

    pub fn get_dataflow_length(&self) -> u16 {
        // TODO: search reduce or fold
        let mut acc = 0;

        for field in &self.fields {
            acc += field.length
        }

        acc
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
    pub flowset_id: u16,
    pub length: u16,
    pub record_bytes: Option<Vec<u8>>,
    pub records: Option<Vec<Vec<FlowField>>>, // TODO: extract as Record
}

// pub struct Record {
//     pub data_flow: Vec<DataFlow>,
// }
// TODO: impl search or map like access API

impl DataFlow {
    pub fn new(
        flowset_id: u16,
        length: u16,
        record_bytes: Option<Vec<u8>>,
        records: Option<Vec<Vec<FlowField>>>,
    ) -> DataFlow {
        DataFlow {
            flowset_id: flowset_id,
            length: length,
            record_bytes: record_bytes,
            records: records,
        }
    }

    // Some implementation seems not to append padding
    pub fn from_bytes_notemplate(data: &[u8]) -> Result<(&[u8], DataFlow), ()> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, length) = flowset_length(&rest).unwrap();
        let length = length.unwrap().1;
        let record_bytes = &rest[..(length as usize - 4)];
        let rest = &rest[(length as usize - 4)..];

        Ok((
            rest,
            DataFlow::new(
                flowset_id.unwrap().1,
                length,
                Some(record_bytes.to_vec()),
                None,
            ),
        ))
    }

    fn get_template(flowset_id: u16, templates: &[DataTemplate]) -> Option<&DataTemplate> {
        let template: Vec<&DataTemplate> = templates
            .iter()
            .filter(|temp| temp.template_id == flowset_id)
            .collect();

        if template.len() == 0 {
            None
        } else {
            Some(&template[0])
        }
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
        let template: Option<&DataTemplate> = DataFlow::get_template(flowset_id, templates);

        match template {
            Some(template) => {
                let records_num = DataFlow::get_record_num(length, template.get_dataflow_length());
                let mut records: Vec<Vec<FlowField>> = Vec::with_capacity(records_num);
                let mut rest = rest;

                for _ in 0..records_num {
                    let (next, fields) = template.parse_dataflow(rest).unwrap();
                    records.push(fields);
                    rest = next;
                }

                let padding = DataFlow::get_padding(length, template.get_dataflow_length());

                if padding > 0 {
                    rest = &rest[(padding as usize)..]
                }

                // Left bytes for future parsing?
                Ok((
                    rest,
                    DataFlow::new(flowset_id, length, None, Some(records)),
                ))
            }
            None => {
                // Return Err, None or bytes field?
                debug!("Template is not found, flowset_id = {}", flowset_id);
                Err(())
            }
        }
    }

    fn get_record_num(payload_len: u16, template_len: u16) -> usize {
        ((payload_len - 4) / template_len) as usize
    }

    fn get_padding(payload_len: u16, template_len: u16) -> u16 {
        payload_len - template_len * DataFlow::get_record_num(payload_len, template_len) as u16 - 4
    }
}
