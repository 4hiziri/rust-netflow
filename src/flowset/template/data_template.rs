use super::DataTemplateItem;
use crate::error::{NetFlowError, ParseResult};
use crate::util::{take_u16, u16_to_bytes};

pub const TEMPLATE_FLOWSET_ID: u16 = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub templates: Vec<DataTemplateItem>,
}

impl DataTemplate {
    const HEADER_LEN: u16 = 4;

    pub fn new(templates: Vec<DataTemplateItem>) -> DataTemplate {
        let length: u16 = templates
            .as_slice()
            .into_iter()
            .fold(0, |sum, temp| sum + temp.byte_length()) as u16;

        DataTemplate {
            flowset_id: TEMPLATE_FLOWSET_ID,
            length,
            templates,
        }
    }

    pub fn from_bytes(data: &[u8]) -> ParseResult<DataTemplate> {
        let (rest, flowset_id) = take_u16(&data)?;
        let (rest, flowset_length) = take_u16(&rest)?;
        let (rest, templates) =
            DataTemplateItem::parse_bytes(flowset_length - Self::HEADER_LEN, &rest)?;

        if flowset_id == TEMPLATE_FLOWSET_ID {
            Ok((
                rest,
                DataTemplate {
                    flowset_id: 0,
                    length: flowset_length,
                    templates,
                },
            ))
        } else {
            Err(NetFlowError::InvalidLength)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];

        u16_to_bytes(self.flowset_id, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        let mut template_bytes = Vec::new();
        for template in &self.templates {
            template_bytes.append(&mut template.to_bytes());
        }

        let length = template_bytes.len() as u16 + Self::HEADER_LEN;

        u16_to_bytes(length, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        bytes.append(&mut template_bytes);

        bytes
    }

    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }
}

#[cfg(test)]
mod data_template_test {
    use super::DataTemplate;
    use crate::error::ParseResult;
    use crate::flowset::test_data;

    #[test]
    fn test_data_template() {
        let data_template_payload = &test_data::TEMPLATE_DATA[..];

        // parsing process test
        let template: ParseResult<DataTemplate> = DataTemplate::from_bytes(&data_template_payload);
        assert!(template.is_ok());

        // parsing result test
        let (_rest, template): (&[u8], DataTemplate) = template.unwrap();
        assert_eq!(template.flowset_id, 0);
        assert_eq!(template.length, 92);

        assert_eq!(template.templates.len(), 1);
        assert_eq!(template.templates[0].template_id, 1024);
        assert_eq!(template.templates[0].field_count, 21);
        assert_eq!(
            template.templates[0].fields.len() as u16,
            template.templates[0].field_count
        );

        // TODO: Field test
    }

    #[test]
    fn test_to_bytes() {
        let test_data = &test_data::TEMPLATE_DATA[..];
        let (_, template) = DataTemplate::from_bytes(&test_data).unwrap();
        let bytes = template.to_bytes();

        assert_eq!(bytes.len() % 4, 0);
        assert_eq!(&bytes.as_slice(), &test_data);
    }

    #[test]
    fn test_convert() {
        let test_data = &test_data::TEMPLATE_DATA[..];
        let (_, template) = DataTemplate::from_bytes(&test_data).unwrap();
        let bytes = template.to_bytes();

        let (_, template) = DataTemplate::from_bytes(&bytes).unwrap();
        let re_bytes = template.to_bytes();
        assert_eq!(re_bytes, bytes);
    }

    #[test]
    fn test_byte_length() {
        let test_data = &test_data::TEMPLATE_DATA[..];
        let (_, template) = DataTemplate::from_bytes(&test_data).unwrap();

        assert_eq!(template.byte_length(), test_data.len());
    }

}
