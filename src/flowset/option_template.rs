use nom;
use nom::be_u16;
use field::{TypeLengthField, FlowField};
use super::{flowset_id, flowset_length, template_id, Template};

pub const OPTION_FLOWSET_ID: u16 = 1;

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

impl Template for OptionTemplate {
    fn get_template_len(&self) -> u16 {
        self.scopes[..].into_iter().fold(
            0,
            |sum, field| sum + field.length,
        ) +
            self.options[..].into_iter().fold(
                0,
                |sum, field| sum + field.length,
            )
    }

    fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> Result<(&'a [u8], Vec<FlowField>), ()> {
        let mut rest = payload;
        let mut fields: Vec<FlowField> = Vec::with_capacity(self.scopes.len() + self.options.len());

        for field in &self.scopes {
            let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, rest)
                .unwrap();

            fields.push(flow_field);
            rest = next;
        }

        for field in &self.options {
            let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, rest)
                .unwrap();

            fields.push(flow_field);
            rest = next;
        }

        Ok((rest, fields))
    }
}
