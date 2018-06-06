use error::ParseResult;
use flowset::Record;

pub trait TemplateParser {
    fn get_id(&self) -> u16;
    fn parse_dataflow<'a>(&self, payload: &'a [u8]) -> ParseResult<'a, Record>;
    fn get_template_len(&self) -> u16;

    fn parse_dataflows<'a>(&self, length: u16, payload: &'a [u8]) -> ParseResult<'a, Vec<Record>> {
        let record_count = length / self.get_template_len();
        let mut record_vec = Vec::with_capacity(record_count as usize);
        let mut rest = payload;

        for _ in 0..record_count {
            let (next, rec) = self.parse_dataflow(rest).unwrap();
            record_vec.push(rec);
            rest = next;
        }

        Ok((rest, record_vec))
    }
}
