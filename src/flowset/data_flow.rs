use super::{Record, TemplateParser};
use error::{Error, ParseResult};
use util::take_u16;

#[derive(Debug)]
pub struct DataFlow {
    pub flowset_id: u16,
    pub length: u16,
    pub record_bytes: Option<Vec<u8>>,
    pub records: Option<Vec<Record>>,
}

// TODO: impl search or map like access API

impl DataFlow {
    pub fn new(
        flowset_id: u16,
        length: u16,
        record_bytes: Option<Vec<u8>>,
        records: Option<Vec<Record>>,
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

    fn remove_padding(length: u16, template_len: u16, payload: &[u8]) -> &[u8] {
        let padding = DataFlow::get_padding(length, template_len);
        let mut rest = payload;

        if padding > 0 {
            rest = &rest[(padding as usize)..]
        }

        rest
    }

    pub fn from_bytes<'a, T>(data: &'a [u8], templates: &'a [T]) -> ParseResult<'a, DataFlow>
    where
        T: TemplateParser,
    {
        debug!("Length of parsing data: {}", data.len());
        let (rest, flowset_id) = take_u16(&data).unwrap();
        let (rest, length) = take_u16(&rest).unwrap();

        let hit_temp: Vec<&T> = templates
            .iter()
            .filter(|template| template.get_id() == flowset_id)
            .collect();

        if hit_temp.len() >= 1 {
            let template = hit_temp[0];

            // sub id and length field's length
            let (rest, records) = template.parse_dataflows(length - 4, rest).unwrap();

            let rest = DataFlow::remove_padding(length, template.get_template_len(), rest);

            Ok((rest, DataFlow::new(flowset_id, length, None, Some(records))))
        } else {
            Err(Error::TemplateNotFound)
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
    use flowset::{test_data, DataTemplateItem};

    #[test]
    fn test_dataflow_no_template() {
        let packet_bytes = test_data::DATAFLOW_DATA;
        let res = DataFlow::from_bytes_notemplate(&packet_bytes);
        assert!(res.is_ok());
    }

    // TODO: move upper combination test?
    #[test]
    fn test_dataflow_template() {
        let (len, data) = test_data::TEMPLATE_FIELDS;
        let (_rest, temp) = DataTemplateItem::from_bytes(len, &data).unwrap();
        let temps = [temp];

        let packet_bytes = test_data::DATAFLOW_DATA;
        let dataflow = DataFlow::from_bytes(&packet_bytes, &temps);
        assert!(dataflow.is_ok());
    }
}
