use nom;
use nom::be_u16;
use field::{TypeLengthField, FlowField};
use super::{flowset_id, flowset_length, template_id};

pub const TEMPLATE_FLOWSET_ID: u16 = 0;

// TODO: need mut?
#[derive(Debug)]
pub struct DataTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub template_id: u16,
    pub field_count: u16,
    pub fields: Vec<TypeLengthField>,
}

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
