use field::FlowField;
use field::TypeLengthField;

pub trait Template {
    fn get_template_len(&self) -> u16;
    fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> Result<(&'a [u8], Vec<FlowField>), ()>;
}
