use error::{Error, ParseResult};
use field::{FlowField, TypeLengthField};
use flowset::{Record, TemplateParser};
use util::take_u16;

#[derive(Debug)]
pub struct OptionTemplateItem {
    pub template_id: u16,
    pub scope_count: u16,
    pub option_count: u16,
    pub scopes: Vec<TypeLengthField>,
    pub options: Vec<TypeLengthField>,
}

impl OptionTemplateItem {
    const HEADER_LEN: u16 = 6;

    pub fn new(
        template_id: u16,
        scope_count: u16,
        option_count: u16,
        scopes: Vec<TypeLengthField>,
        options: Vec<TypeLengthField>,
    ) -> Self {
        OptionTemplateItem {
            template_id: template_id,
            scope_count: scope_count,
            option_count: option_count,
            scopes: scopes,
            options: options,
        }
    }

    pub fn from_bytes(length: u16, data: &[u8]) -> ParseResult<OptionTemplateItem> {
        let (rest, template_id) = take_u16(&data).unwrap();
        let (rest, scope_length) = take_u16(&rest).unwrap();
        let (rest, option_length) = take_u16(&rest).unwrap();

        if (length - OptionTemplateItem::HEADER_LEN) >= (scope_length + option_length) {
            let scope_count = scope_length / 4; // TODO: remove mgk num
            let (rest, scopes): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::to_vec(scope_count as usize, &rest).unwrap();

            let option_count = option_length / 4;
            let (rest, options): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::to_vec(option_count as usize, &rest).unwrap();

            // remove padding
            Ok((
                if length - OptionTemplateItem::HEADER_LEN - scope_length - option_length != 0 {
                    let pad_len: usize =
                        (length - OptionTemplateItem::HEADER_LEN - scope_length - option_length)
                            as usize;
                    &rest[pad_len..]
                } else {
                    rest
                },
                OptionTemplateItem::new(template_id, scope_count, option_count, scopes, options),
            ))
        } else {
            Err(Error::InvalidLength)
        }
    }

    fn get_fields_len(&self) -> u16 {
        self.scope_count * 4 + self.option_count * 4
    }

    pub fn to_vec(length: u16, data: &[u8]) -> ParseResult<Vec<OptionTemplateItem>> {
        let mut templates: Vec<Self> = Vec::new();
        let mut rest_length = length;
        let mut rest = data;

        debug!("rest_length = {:?}", rest_length);

        while rest_length > 0 {
            let (next, template) = OptionTemplateItem::from_bytes(rest_length, rest).unwrap();
            rest_length -= OptionTemplateItem::HEADER_LEN + template.get_fields_len();
            templates.push(template);
            rest = next;
        }

        Ok((rest, templates))
    }
}

impl TemplateParser for OptionTemplateItem {
    fn get_id(&self) -> u16 {
        self.template_id
    }

    // field_count is u16 and a entry is 4-bytes len.
    fn get_template_len(&self) -> u16 {
        self.scopes[..].into_iter().fold(0, |sum, i| sum + i.length)
            + self.options[..]
                .into_iter()
                .fold(0, |sum, i| sum + i.length)
    }

    fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> ParseResult<'a, Record> {
        let mut rest = payload;
        let mut scopes: Vec<FlowField> = Vec::with_capacity(self.scopes.len());

        for field in &self.scopes {
            let (next, flow_field) =
                FlowField::from_bytes(field.type_id, field.length, rest).unwrap();

            scopes.push(flow_field);
            rest = next;
        }

        let mut options: Vec<FlowField> = Vec::with_capacity(self.options.len());
        for field in &self.options {
            let (next, flow_field) =
                FlowField::from_bytes(field.type_id, field.length, rest).unwrap();

            options.push(flow_field);
            rest = next;
        }

        Ok((rest, Record::make_option(scopes, options)))
    }
}

#[cfg(test)]
mod option_template_test {
    use super::OptionTemplateItem;
    use flowset::{test_data, TemplateParser};

    #[test]
    fn test_parse() {
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let (rest, temp): (&[u8], OptionTemplateItem) =
            OptionTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(temp.scopes.len() + temp.options.len(), 4);
    }

    #[test]
    fn test_get_len() {
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let (_rest, temp): (&[u8], OptionTemplateItem) =
            OptionTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(temp.get_template_len(), 10);
        assert_eq!(temp.get_fields_len(), 16);
    }

    #[test]
    fn test_to_vec() {
        // TODO: new data and more test
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let (rest, temp) = OptionTemplateItem::to_vec(len, &data).unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(temp.len(), 1);
        assert_eq!(temp[0].scopes.len() + temp[0].options.len(), 4);
    }

    #[test]
    fn test_get_id() {
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let (_rest, temp) = OptionTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(temp.get_id(), 4096);
    }

    #[test]
    fn test_parse_dataflow() {
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let dataflow = test_data::OPTION_TEMPLATE_ITEM_DATAFLOW;
        let (_rest, temp) = OptionTemplateItem::from_bytes(len, &data).unwrap();

        let record = temp.parse_dataflow(&dataflow);
        assert!(record.is_ok());
        // TODO: check field
    }
}