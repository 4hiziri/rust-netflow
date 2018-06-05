use super::Template;
use field::TypeLengthField;
use util::take_u16;

pub const OPTION_FLOWSET_ID: u16 = 1;

// #[derive(Debug)]
// pub struct OptionTemplate {
//     pub flowset_id: u16,
//     pub length: u16,
//     pub template_id: u16,
//     pub option_scope_length: u16,
//     pub option_length: u16,
//     pub scopes: Vec<TypeLengthField>,
//     pub options: Vec<TypeLengthField>,
// }

// TODO: change more easy struct
#[derive(Debug)]
pub struct OptionTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub scope_template: Template,
    pub option_template: Template,
}

impl OptionTemplate {
    pub fn new(
        flowset_id: u16,
        length: u16,
        template_id: u16,
        scope_len: u16,
        opt_len: u16,
        scopes: Vec<TypeLengthField>,
        options: Vec<TypeLengthField>,
    ) -> Self {
        let scope = Template::new(template_id, scope_len / 4, scopes);
        let option = Template::new(template_id, opt_len / 4, options);

        OptionTemplate {
            flowset_id: flowset_id,
            length: length,
            scope_template: scope,
            option_template: option,
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<(&[u8], OptionTemplate), ()> {
        let (rest, flowset_id) = take_u16(&data).unwrap();

        if flowset_id == OPTION_FLOWSET_ID {
            let (rest, length) = take_u16(&rest).unwrap();
            let (rest, template_id) = take_u16(&rest).unwrap();
            let (rest, scope_len) = take_u16(&rest).unwrap();
            let (rest, option_len) = take_u16(&rest).unwrap();
            let (rest, scopes) =
                TypeLengthField::take_from((scope_len / 4) as usize, rest).unwrap();
            let (rest, options) =
                TypeLengthField::take_from((option_len / 4) as usize, rest).unwrap();

            Ok((
                rest,
                OptionTemplate::new(
                    flowset_id,
                    length,
                    template_id,
                    scope_len,
                    option_len,
                    scopes,
                    options,
                ),
            ))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test_option_template {
    use super::OptionTemplate;
    use flowset::test_data;

    #[test]
    fn test_option_template() {
        let packet_bytes = &test_data::OPTION_DATA[..];

        let option: Result<(&[u8], OptionTemplate), ()> = OptionTemplate::from_bytes(&packet_bytes);
        assert!(option.is_ok());

        let (_rest, option): (&[u8], OptionTemplate) = option.unwrap();
        assert_eq!(option.flowset_id, 1);
        assert_eq!(option.length, 26);
        assert_eq!(option.scope_template.template_id, 4096);
        assert_eq!(option.option_template.template_id, 4096);
        assert_eq!(option.scope_template.field_count, 1);
        assert_eq!(option.option_template.field_count, 3);
    }

}
