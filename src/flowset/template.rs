use field::{FlowField, TypeLengthField};
use util::take_u16;

#[derive(Debug)]
pub struct Template {
    pub template_id: u16,
    pub field_count: u16,
    pub fields: Vec<TypeLengthField>,
}

impl Template {
    const HEADER_LEN: u16 = 4;

    pub fn new(template_id: u16, field_count: u16, fields: Vec<TypeLengthField>) -> Self {
        Template {
            template_id: template_id,
            field_count: field_count,
            fields: fields,
        }
    }

    pub fn from_bytes(length: u16, data: &[u8]) -> Result<(&[u8], Self), ()> {
        let (rest, template_id) = take_u16(&data).unwrap();
        let (rest, field_count) = take_u16(&rest).unwrap();
        let field_count = field_count.unwrap().1;

        debug!("field_count is {}", field_count);

        // template_id, field_count, flowset_id and flowset_length field len is 8
        if length - Template::HEADER_LEN >= field_count * 4 {
            let (rest, fields): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::take_from(field_count as usize, &rest).unwrap(); // Err

            Ok((
                rest,
                Template::new(template_id.unwrap().1, field_count, fields),
            ))
        } else {
            Err(())
        }
    }

    // field_count is u16 and a entry is 4-bytes len.
    pub fn get_template_len(&self) -> u16 {
        self.fields[..].into_iter().fold(0, |sum, i| sum + i.length)
    }

    fn get_fields_len(&self) -> u16 {
        self.field_count * 4
    }

    pub fn to_vec(length: u16, data: &[u8]) -> Result<(&[u8], Vec<Self>), ()> {
        let mut templates: Vec<Self> = Vec::new();
        let mut rest_length = length;
        let mut rest = data;

        debug!("rest_length = {:?}", rest_length);

        while rest_length > 0 {
            let (next, template) = Template::from_bytes(rest_length, rest).unwrap();
            rest_length -= Template::HEADER_LEN + template.get_fields_len();
            templates.push(template);
            rest = next;
        }

        Ok((rest, templates))
    }

    pub fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> Result<(&'a [u8], Vec<FlowField>), ()> {
        let mut rest = payload;
        let template = &self.fields;
        let mut fields: Vec<FlowField> = Vec::with_capacity(template.len());

        for field in template {
            let (next, flow_field) =
                FlowField::from_bytes(field.type_id, field.length, rest).unwrap();

            fields.push(flow_field);
            rest = next;
        }

        Ok((rest, fields))
    }
}

#[cfg(test)]
mod template_test {
    use super::Template;
    use flowset::test_data;

    #[test]
    fn test_parse() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (rest, temp) = Template::from_bytes(len, &data).unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(temp.fields.len(), 21);
    }

    #[test]
    fn test_get_template_len() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (_rest, temp) = Template::from_bytes(len, &data).unwrap();

        assert_eq!(temp.get_template_len(), 60);
    }

    #[test]
    fn test_to_vec() {
        // TODO: new data
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (rest, temp) = Template::to_vec(len, &data).unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(temp[0].fields.len(), 21);
    }
}
