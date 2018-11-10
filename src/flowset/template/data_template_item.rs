use crate::error::{Error, ParseResult};
use crate::field::{FlowField, TypeLengthField};
use crate::flowset::{Record, TemplateParser};
use crate::util::{take_u16, u16_to_bytes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTemplateItem {
    pub template_id: u16,
    pub field_count: u16,
    pub fields: Vec<TypeLengthField>,
}

impl DataTemplateItem {
    const HEADER_LEN: u16 = 4; // len(id + length) = 4

    pub fn new(template_id: u16, fields: Vec<TypeLengthField>) -> DataTemplateItem {
        DataTemplateItem {
            template_id,
            field_count: fields.len() as u16,
            fields,
        }
    }

    /// Return DataTemplateItem from data
    /// length is DataTemplateItem's length, not DataTemplate's
    /// validate with length, need this?
    pub fn from_bytes(length: u16, data: &[u8]) -> ParseResult<DataTemplateItem> {
        let (rest, template_id) = take_u16(&data)?;
        let (rest, field_count) = take_u16(&rest)?;

        if length - DataTemplateItem::HEADER_LEN >= field_count * 4 {
            let (rest, fields): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::parse_bytes(field_count as usize, &rest)?;

            Ok((
                rest,
                DataTemplateItem {
                    template_id,
                    field_count,
                    fields,
                },
            ))
        } else {
            Err(Error::InvalidLength)
        }
    }

    fn get_fields_len(&self) -> u16 {
        self.field_count * 4
    }

    pub fn parse_bytes(length: u16, data: &[u8]) -> ParseResult<Vec<DataTemplateItem>> {
        let mut templates: Vec<Self> = Vec::new();
        let mut rest_length = length;
        let mut rest = data;

        debug!("rest_length = {:?}", rest_length);

        while rest_length > 0 {
            let (next, template) = DataTemplateItem::from_bytes(rest_length, rest)?;
            rest_length -= DataTemplateItem::HEADER_LEN + template.get_fields_len();
            templates.push(template);
            rest = next;
        }

        Ok((rest, templates))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];

        u16_to_bytes(self.template_id, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u16_to_bytes(self.field_count, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        for field in &self.fields {
            bytes.append(&mut field.to_bytes());
        }

        // data template doesn't have padding

        bytes
    }

    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
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
            let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, &rest)?;

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
    use crate::flowset::test_data;

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
    fn test_parse_bytes() {
        // TODO: new data
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (rest, temp) = DataTemplateItem::parse_bytes(len, &data).unwrap();

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

    #[test]
    fn test_to_bytes() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (_rest, temp) = DataTemplateItem::from_bytes(len, &data).unwrap();
        let bytes = temp.to_bytes();

        assert_eq!(&bytes.as_slice(), &data.as_ref());
    }

    #[test]
    fn test_byte_length() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (_rest, temp) = DataTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(temp.byte_length(), data.len());
    }
}
