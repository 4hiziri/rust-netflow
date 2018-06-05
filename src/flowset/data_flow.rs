use super::{DataTemplate, Template};
use error::{Error, ParseResult};
use field::FlowField;
use util::take_u16;

#[derive(Debug)]
pub struct DataFlow {
    pub flowset_id: u16,
    pub length: u16,
    pub record_bytes: Option<Vec<u8>>,
    pub records: Option<Vec<Vec<FlowField>>>, // TODO: extract as Record
}

// pub struct Record {
//     pub data_flow: Vec<DataFlow>,
// }
// TODO: impl search or map like access API

impl DataFlow {
    pub fn new(
        flowset_id: u16,
        length: u16,
        record_bytes: Option<Vec<u8>>,
        records: Option<Vec<Vec<FlowField>>>,
    ) -> DataFlow {
        DataFlow {
            flowset_id: flowset_id,
            length: length,
            record_bytes: record_bytes,
            records: records,
        }
    }

    // Some implementation seems not to append padding
    pub fn from_bytes_notemplate(data: &[u8]) -> ParseResult<DataFlow> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = take_u16(&data).unwrap();
        let (rest, length) = take_u16(&rest).unwrap();
        let record_bytes = &rest[..(length as usize - 4)];
        let rest = &rest[(length as usize - 4)..];

        Ok((
            rest,
            DataFlow::new(flowset_id, length, Some(record_bytes.to_vec()), None),
        ))
    }

    fn get_template(flowset_id: u16, templates: &[DataTemplate]) -> Option<&Template> {
        // TODO: flat templates to Vec<Template> like object
        let template: Vec<&Template> = templates
            .into_iter()
            .flat_map(|data_temp| &data_temp.templates)
            .filter(|temp| temp.template_id == flowset_id)
            .collect();

        if template.len() == 0 {
            None
        } else {
            Some(template[0])
        }
    }

    // TODO: extract Result<(u8, ~), ()> as someResult
    fn parse_records<'a>(
        template: &Template,
        records_num: usize, // TODO: check max length
        payload: &'a [u8],
    ) -> ParseResult<'a, Vec<Vec<FlowField>>> {
        let mut records: Vec<Vec<FlowField>> = Vec::with_capacity(records_num);
        let mut rest = payload;

        for _ in 0..records_num {
            let (next, fields) = template.parse_dataflow(rest).unwrap(); // Err
            records.push(fields);
            rest = next;
        }

        Ok((rest, records))
    }

    fn remove_padding(length: u16, template_len: u16, payload: &[u8]) -> &[u8] {
        let padding = DataFlow::get_padding(length, template_len);
        let mut rest = payload;

        if padding > 0 {
            rest = &rest[(padding as usize)..]
        }

        rest
    }

    pub fn from_bytes<'a>(data: &'a [u8], templates: &[DataTemplate]) -> ParseResult<'a, DataFlow> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = take_u16(&data).unwrap();
        let flowset_id = flowset_id;
        let (rest, length) = take_u16(&rest).unwrap();
        let length = length;

        let template = DataFlow::get_template(flowset_id, templates);

        match template {
            Some(template) => {
                let template_len = template.get_template_len();
                let (rest, records) = DataFlow::parse_records(
                    template,
                    DataFlow::get_record_num(length, template_len),
                    rest,
                ).unwrap();

                let rest = DataFlow::remove_padding(length, template_len, rest);

                Ok((rest, DataFlow::new(flowset_id, length, None, Some(records))))
            }
            None => {
                debug!("Template is not found, flowset_id = {}", flowset_id);
                Err(Error::InvalidLength)
            }
        }
    }

    fn get_record_num(payload_len: u16, template_len: u16) -> usize {
        ((payload_len - 4) / template_len) as usize
    }

    fn get_padding(payload_len: u16, template_len: u16) -> u16 {
        payload_len - template_len * DataFlow::get_record_num(payload_len, template_len) as u16 - 4
    }
}

#[cfg(test)]
mod test_data_flow {
    use super::DataFlow;
    use flowset::test_data;

    #[test]
    fn test_data_flow() {
        let packet_bytes = &test_data::DATAFLOW_DATA[..];

        let res = DataFlow::from_bytes_notemplate(&packet_bytes);
        assert!(res.is_ok());
    }
}
