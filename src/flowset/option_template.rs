use super::OptionTemplateItem;
use error::{Error, ParseResult};
use field::TypeLengthField;
use util::{take_u16, u16_to_bytes};

pub const OPTION_FLOWSET_ID: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub templates: OptionTemplateItem,
}

// TODO: add field that represent padding
impl OptionTemplate {
    pub fn new(
        template_id: u16,
        scopes: Vec<TypeLengthField>,
        options: Vec<TypeLengthField>,
    ) -> OptionTemplate {
        let template = OptionTemplateItem::new(template_id, scopes, options);
        let length = 10 + template.option_count * 4 + template.scope_count * 4;

        // TODO: add padding
        OptionTemplate {
            flowset_id: 1,
            length: length,
            templates: template,
        }
    }

    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        let (rest, flowset_id) = take_u16(&data).unwrap();

        if flowset_id == OPTION_FLOWSET_ID {
            let (rest, length) = take_u16(&rest).unwrap();
            let (rest, option_item) = OptionTemplateItem::from_bytes(length - 4, rest).unwrap();
            // TODO: error handling

            Ok((
                rest,
                OptionTemplate {
                    flowset_id: flowset_id,
                    length: length,
                    templates: option_item,
                },
            ))
        } else {
            Err(Error::InvalidLength)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];

        u16_to_bytes(self.flowset_id, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u16_to_bytes(self.length, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        bytes.append(&mut self.templates.to_bytes());

        debug!("Bytes length before padding: {:?}", bytes.len());
        // padding
        // option template must be over 2 bytem
        bytes.push(0);
        bytes.push(0);

        bytes
    }

    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }
}

#[cfg(test)]
mod test_option_template {
    use super::OptionTemplate;
    use error::ParseResult;
    use flowset::test_data;

    #[test]
    fn test_option_template() {
        let packet_bytes = &test_data::OPTION_DATA[..];
        let option: ParseResult<OptionTemplate> = OptionTemplate::from_bytes(&packet_bytes);
        assert!(option.is_ok());

        let (_rest, option): (&[u8], OptionTemplate) = option.unwrap();
        assert_eq!(option.flowset_id, 1);
        assert_eq!(option.length, 26);
        assert_eq!(option.templates.template_id, 4096);
        assert_eq!(option.templates.scope_count, 1);
        assert_eq!(option.templates.option_count, 3);
    }

    #[test]
    fn test_to_bytes() {
        // TODO: check padding specification
        let packet_bytes = &test_data::OPTION_DATA[..];
        let (_rest, option) = OptionTemplate::from_bytes(&packet_bytes).unwrap();
        let bytes = option.to_bytes();

        assert_eq!(bytes.len() % 4, 0);
        assert_eq!(&bytes.as_slice(), &packet_bytes);
    }

    #[test]
    fn test_convert() {
        let packet_bytes = &test_data::OPTION_DATA[..];
        let (_rest, option) = OptionTemplate::from_bytes(&packet_bytes).unwrap();
        let bytes = option.to_bytes();

        let (_rest, option) = OptionTemplate::from_bytes(&bytes).unwrap();
        let re_bytes = option.to_bytes();

        assert_eq!(re_bytes, bytes);
    }

    #[test]
    fn test_byte_length() {
        // TODO: check padding specification
        let packet_bytes = &test_data::OPTION_DATA[..];
        let (_rest, option) = OptionTemplate::from_bytes(&packet_bytes).unwrap();

        assert_eq!(option.byte_length(), packet_bytes.len());
    }
}
