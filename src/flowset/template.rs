use field::FlowField;
use field::TypeLengthField;

pub trait Template {
    // fn get_template<'a>(&'a self) -> &'a [TypeLengthField];
    fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> Result<(&'a [u8], Vec<FlowField>), ()>;
}
