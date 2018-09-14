use super::OptionTemplateItem;
use error::{Error, ParseResult};
use util::{take_u16, u16_to_bytes};

pub const OPTION_FLOWSET_ID: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub templates: OptionTemplateItem,
    is_padding: bool,
}

impl OptionTemplate {
    const HEADER_LEN: u16 = 4; // id + length + id + scope_count + option_count

    pub fn get_header_len() -> u16 {
        Self::HEADER_LEN
    }

    pub fn new(template: OptionTemplateItem) -> OptionTemplate {
        let length = Self::get_header_len()
            + OptionTemplateItem::get_header_len()
            + template.option_count * 4
            + template.scope_count * 4;

        OptionTemplate {
            flowset_id: OPTION_FLOWSET_ID,
            length: length,
            templates: template,
            is_padding: true,
        }
    }

    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        let (rest, flowset_id) = take_u16(&data)?;

        if flowset_id == OPTION_FLOWSET_ID {
            let (rest, length) = take_u16(&rest)?;
            let (rest, option_item) = OptionTemplateItem::from_bytes(length - 4, rest)?;
            // if payload length is not multiple of 4 and length is multiple of 4, padding exists
            let is_padding =
                option_item.scope_count + option_item.option_count % 2 == 0 && length % 4 == 0;
            // TODO: extract?

            Ok((
                rest,
                OptionTemplate {
                    flowset_id: flowset_id,
                    length: length,
                    templates: option_item,
                    is_padding: is_padding,
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

        // TODO: add length
        // u16_to_bytes(self.length, &mut u16_buf);
        // bytes.append(&mut u16_buf.to_vec());

        let mut option = self.templates.to_bytes();
        // bytes.append(&mut self.templates.to_bytes());

        // option's padding is always 2 bytes
        if self.is_padding() && option.len() % 4 != 0 {
            option.push(0);
            option.push(0);
        }

        // add length, calculated from option body
        let length = Self::get_header_len() + option.len() as u16;
        u16_to_bytes(length, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        bytes.append(&mut option);

        bytes
    }

    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }

    pub fn is_padding(&self) -> bool {
        self.is_padding
    }

    pub fn set_padding(&mut self, flag: bool) {
        self.is_padding = flag;
    }
}

#[cfg(test)]
mod test_option_template {
    use super::OptionTemplate;
    use error::ParseResult;
    use flowset::test_data;

    #[test]
    fn test_option_template() {
        let (packet_bytes, _) = test_data::OPTION_DATA;
        let option: ParseResult<OptionTemplate> = OptionTemplate::from_bytes(&packet_bytes);
        assert!(option.is_ok());

        let (_rest, mut option): (&[u8], OptionTemplate) = option.unwrap();
        assert_eq!(option.flowset_id, 1);
        assert_eq!(option.length, 26);
        assert_eq!(option.templates.template_id, 4096);
        assert_eq!(option.templates.scope_count, 1);
        assert_eq!(option.templates.option_count, 3);
        assert!(!option.is_padding());
        option.set_padding(true);
        assert!(option.is_padding());
    }

    #[test]
    fn test_to_bytes() {
        let (packet_bytes, padding_bytes) = test_data::OPTION_DATA;
        let (_rest, mut option) = OptionTemplate::from_bytes(&packet_bytes).unwrap();

        option.set_padding(false);
        let bytes = option.to_bytes();
        assert_eq!(&bytes.as_slice(), &packet_bytes);

        // if padding exists
        let mut packet_bytes: Vec<u8> = Vec::from(&padding_bytes[..]);
        option.set_padding(true);
        let bytes = option.to_bytes();
        assert_eq!(bytes.len() % 4, 0);
        assert_eq!(&bytes.as_slice(), &packet_bytes.as_slice());
    }

    #[test]
    fn test_convert() {
        let (packet_bytes, _) = test_data::OPTION_DATA;
        let (_rest, option) = OptionTemplate::from_bytes(&packet_bytes).unwrap();
        let bytes = option.to_bytes();

        let (_rest, option) = OptionTemplate::from_bytes(&bytes).unwrap();
        let re_bytes = option.to_bytes();

        assert_eq!(re_bytes, bytes);
    }

    #[test]
    fn test_byte_length() {
        let (packet_bytes, _) = test_data::OPTION_DATA;
        let (_rest, mut option) = OptionTemplate::from_bytes(&packet_bytes).unwrap();

        assert_eq!(option.byte_length(), packet_bytes.len());

        // if option doesn't have padding
        option.set_padding(true);
        assert_eq!(option.byte_length(), packet_bytes.len() + 2);
        assert_eq!(option.byte_length() % 4, 0);
    }
}
