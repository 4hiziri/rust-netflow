use super::{Record, TemplateParser};
use error::{Error, ParseResult};
use util::{take_u16, u16_to_bytes};

#[derive(Debug)]
pub struct DataFlow {
    pub flowset_id: u16,
    pub length: u16,
    record_bytes: Vec<u8>,
    pub records: Option<Vec<Record>>,
}

// TODO: improve poor struct, make records and record_bytes convert implicity

// TODO: impl search or map like access API

impl DataFlow {
    // TODO: this can make invalid records and record_bytes, I should make another interface?
    pub fn new(flowset_id: u16, length: u16, records: Vec<Record>) -> DataFlow {
        let mut bytes = Vec::new();

        for record in &records {
            bytes.append(&mut record.to_bytes());
        }

        DataFlow {
            flowset_id: flowset_id,
            length: length,
            record_bytes: bytes,
            records: Some(records),
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
            DataFlow {
                flowset_id: flowset_id,
                length: length,
                record_bytes: record_bytes.to_vec(),
                records: None,
            },
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
            let bytes = rest;
            let (rest, records) = template.parse_dataflows(length - 4, rest).unwrap();
            let rest = DataFlow::remove_padding(length, template.get_template_len(), rest);

            Ok((
                rest,
                DataFlow {
                    flowset_id: flowset_id,
                    length: length,
                    record_bytes: bytes[..(length as usize - 4)].to_vec(), // TODO: need?
                    records: Some(records),
                },
            ))
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

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];

        u16_to_bytes(self.flowset_id, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u16_to_bytes(self.length, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        match &self.records {
            Some(records) => {
                for record in records {
                    let mut buf = record.to_bytes();
                    println!("to: {:?}", buf);
                    bytes.append(&mut buf);
                }
            }
            None => bytes.append(&mut self.record_bytes.to_vec()),
        };

        bytes
    }
}

#[cfg(test)]
mod test_data_flow {
    use super::DataFlow;
    use flowset::{test_data, DataTemplate, DataTemplateItem};

    #[test]
    fn test_dataflow_no_template() {
        let packet_bytes = test_data::DATAFLOW_DATA;
        let res = DataFlow::from_bytes_notemplate(&packet_bytes);
        assert!(res.is_ok());
    }

    #[test]
    fn test_dataflow_with_template() {
        let template = DataTemplate::from_bytes(&test_data::TEMPLATE_AND_DATA.0);
        assert!(template.is_ok());
        let template: DataTemplate = template.unwrap().1;

        let dataflow = DataFlow::from_bytes(&test_data::TEMPLATE_AND_DATA.1, &template.templates);
        assert!(dataflow.is_ok());
        let dataflow: DataFlow = dataflow.unwrap().1;

        assert!(dataflow.records.is_some());
        assert_eq!(dataflow.flowset_id, 1024);
        assert_eq!(dataflow.length, 484);

        let records = dataflow.records.unwrap();
        assert_eq!(records.len(), 8);
    }

    #[test]
    fn test_to_bytes() {
        // {
        //     let testdata = test_data::DATAFLOW_DATA;
        //     let (_, dataflow) = DataFlow::from_bytes_notemplate(&testdata).unwrap();
        //     let bytes = dataflow.to_bytes();
        //     assert_eq!(&bytes.as_slice(), &testdata.as_ref());
        // }

        {
            let testdata = test_data::TEMPLATE_AND_DATA.1;
            let template = DataTemplate::from_bytes(&test_data::TEMPLATE_AND_DATA.0)
                .unwrap()
                .1;
            let dataflow = DataFlow::from_bytes(&testdata, &template.templates)
                .unwrap()
                .1;
            let bytes = dataflow.to_bytes();
            assert_eq!(&bytes.as_slice(), &testdata.as_ref());
        }
    }
}
