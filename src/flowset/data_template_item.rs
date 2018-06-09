use error::{Error, ParseResult};
use field::{FlowField, TypeLengthField};
use flowset::{Record, TemplateParser};
use util::take_u16;

#[derive(Debug)]
pub struct DataTemplateItem {
    pub template_id: u16,
    pub field_count: u16,
    pub fields: Vec<TypeLengthField>,
}

impl DataTemplateItem {
    const HEADER_LEN: u16 = 4; // len(id + length) = 4

    pub fn new(
        template_id: u16,
        field_count: u16,
        fields: Vec<TypeLengthField>,
    ) -> DataTemplateItem {
        DataTemplateItem {
            template_id: template_id,
            field_count: field_count,
            fields: fields,
        }
    }

    pub fn from_bytes(length: u16, data: &[u8]) -> ParseResult<DataTemplateItem> {
        let (rest, template_id) = take_u16(&data).unwrap();
        let (rest, field_count) = take_u16(&rest).unwrap();

        debug!("field_count is {}", field_count);

        if length - DataTemplateItem::HEADER_LEN >= field_count * 4 {
            let (rest, fields): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::to_vec(field_count as usize, &rest).unwrap();

            Ok((
                rest,
                DataTemplateItem::new(template_id, field_count, fields),
            ))
        } else {
            Err(Error::InvalidLength)
        }
    }

    fn get_fields_len(&self) -> u16 {
        self.field_count * 4
    }

    pub fn to_vec(length: u16, data: &[u8]) -> ParseResult<Vec<Self>> {
        let mut templates: Vec<Self> = Vec::new();
        let mut rest_length = length;
        let mut rest = data;

        debug!("rest_length = {:?}", rest_length);

        while rest_length > 0 {
            let (next, template) = DataTemplateItem::from_bytes(rest_length, rest).unwrap();
            rest_length -= DataTemplateItem::HEADER_LEN + template.get_fields_len();
            templates.push(template);
            rest = next;
        }

        Ok((rest, templates))
    }
}

impl TemplateParser for DataTemplateItem {
    fn get_id(&self) -> u16 {
        self.template_id
    }

    // field_count is u16 and a entry is 4-bytes len.
    fn get_template_len(&self) -> u16 {
        self.fields[..].into_iter().fold(0, |sum, i| sum + i.length)
    }

    fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> ParseResult<'a, Record> {
        let mut rest: &[u8] = &payload;
        let mut fields: Vec<FlowField> = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let (next, flow_field) =
                FlowField::from_bytes(field.type_id, field.length, &rest).unwrap();

            fields.push(flow_field);
            rest = next;
        }

        let rec = Record::make_data(fields);

        Ok((rest, rec))
    }
}

#[cfg(test)]
mod data_template_test {
    use super::{DataTemplateItem, TemplateParser};
    use flowset::test_data;

    #[test]
    fn test_parse() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (rest, temp) = DataTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(temp.fields.len(), 21);
    }

    #[test]
    fn test_get_template_len() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (_rest, temp): (&[u8], DataTemplateItem) =
            DataTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(temp.template_id, 1024);
        assert_eq!(temp.get_template_len(), 60);
    }

    #[test]
    fn test_to_vec() {
        // TODO: new data
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (rest, temp) = DataTemplateItem::to_vec(len, &data).unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(temp[0].fields.len(), 21);
    }

    #[test]
    fn test_parse_template() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (_rest, temp) = DataTemplateItem::from_bytes(len, &data).unwrap();
        let dataflow = test_data::DATA_TEMPLATE_ITEM_DATA;

        let rec = temp.parse_dataflow(&dataflow);
        assert!(rec.is_ok());
    }
}