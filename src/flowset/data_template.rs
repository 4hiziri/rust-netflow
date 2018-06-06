use super::DataTemplateItem;
use error::{Error, ParseResult};
use util::take_u16;

pub const TEMPLATE_FLOWSET_ID: u16 = 0;

// TODO: need mut?
#[derive(Debug)]
pub struct DataTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub templates: Vec<DataTemplateItem>,
}

impl DataTemplate {
    pub fn new(length: u16, templates: Vec<DataTemplateItem>) -> DataTemplate {
        DataTemplate {
            flowset_id: 0, // DataTemplate's flowset_id is 0
            length: length,
            templates: templates,
        }
    }

    pub fn from_bytes(data: &[u8]) -> ParseResult<DataTemplate> {
        let (rest, flowset_id) = take_u16(&data).unwrap();
        let (rest, flowset_length) = take_u16(&rest).unwrap();
        let (rest, templates) = DataTemplateItem::to_vec(flowset_length - 4, &rest).unwrap();

        if flowset_id == TEMPLATE_FLOWSET_ID {
            Ok((rest, DataTemplate::new(flowset_length, templates)))
        } else {
            Err(Error::InvalidLength)
        }
    }
}

#[cfg(test)]
mod data_template_test {
    use super::DataTemplate;
    use error::ParseResult;
    use flowset::test_data;

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
}
