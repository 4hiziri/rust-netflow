use error::{Error, ParseResult};
use field::{FlowField, TypeLengthField};
use flowset::{Record, TemplateParser};
use util::{take_u16, u16_to_bytes};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        scopes: Vec<TypeLengthField>,
        options: Vec<TypeLengthField>,
    ) -> Self {
        OptionTemplateItem {
            template_id: template_id,
            scope_count: scopes.len() as u16,
            option_count: options.len() as u16,
            scopes: scopes,
            options: options,
        }
    }

    /// Return OptionTemplateItem from data
    /// length is OptionTemplateItem's length, not OptionTemplate's
    /// validate with length, need this?
    pub fn from_bytes(length: u16, data: &[u8]) -> ParseResult<OptionTemplateItem> {
        let (rest, template_id) = take_u16(&data)?;
        let (rest, scope_length) = take_u16(&rest)?;
        let (rest, option_length) = take_u16(&rest)?;

        // TODO: need this check?
        if (length - OptionTemplateItem::HEADER_LEN) >= (scope_length + option_length) {
            let scope_count = scope_length / 4; // TODO: remove mgk num
            let (rest, scopes): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::to_vec(scope_count as usize, &rest)?;

            let option_count = option_length / 4;
            let (rest, options): (&[u8], Vec<TypeLengthField>) =
                TypeLengthField::to_vec(option_count as usize, &rest)?;

            // remove padding
            Ok((
                // TODO: extract as function
                if length - OptionTemplateItem::HEADER_LEN - scope_length - option_length != 0 {
                    let pad_len: usize =
                        (length - OptionTemplateItem::HEADER_LEN - scope_length - option_length)
                            as usize;
                    &rest[pad_len..]
                } else {
                    rest
                },
                OptionTemplateItem {
                    template_id: template_id,
                    scope_count: scope_count,
                    option_count: option_count,
                    scopes: scopes,
                    options: options,
                },
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
            let (next, template) = OptionTemplateItem::from_bytes(rest_length, rest)?;
            rest_length -= OptionTemplateItem::HEADER_LEN + template.get_fields_len();
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

        u16_to_bytes(self.scope_count * 4, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u16_to_bytes(self.option_count * 4, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        for scope in &self.scopes {
            bytes.append(&mut scope.to_bytes());
        }

        for option in &self.options {
            bytes.append(&mut option.to_bytes());
        }

        bytes
    }

    // TODO: extract as trait
    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
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
            let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, rest)?;

            scopes.push(flow_field);
            rest = next;
        }

        let mut options: Vec<FlowField> = Vec::with_capacity(self.options.len());
        for field in &self.options {
            let (next, flow_field) = FlowField::from_bytes(field.type_id, field.length, rest)?;

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

    #[test]
    fn test_to_bytes() {
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let (_rest, temp) = OptionTemplateItem::from_bytes(len, &data).unwrap();
        let bytes = temp.to_bytes();

        assert_eq!(&bytes.as_slice(), &data);
    }

    #[test]
    fn test_byte_length() {
        let (len, data) = test_data::OPTION_TEMPLATE_ITEM;
        let (_rest, temp) = OptionTemplateItem::from_bytes(len, &data).unwrap();

        assert_eq!(temp.byte_length(), data.len());
    }
}
