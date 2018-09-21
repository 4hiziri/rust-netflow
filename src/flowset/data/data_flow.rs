use super::Record;
use error::{Error, ParseResult};
use flowset::TemplateParser;
use util::{take_u16, u16_to_bytes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    pub flowset_id: u16,
    pub length: u16,
    record_bytes: Vec<u8>, // how serialized?
    pub records: Option<Vec<Record>>,
    is_padding: bool, // in default, padding SHOULD be enabled.
}

// TODO: impl search or map like access API

impl DataFlow {
    const HEADER_LEN: u16 = 4; // id(2 byte) + length(2 bytes)

    pub fn new(flowset_id: u16, records: Vec<Record>) -> DataFlow {
        // TODO: need sanity check?
        let mut bytes = Vec::new();

        for record in &records {
            bytes.append(&mut record.to_bytes());
        }

        DataFlow {
            flowset_id,
            length: Self::HEADER_LEN + bytes.len() as u16,
            record_bytes: bytes,
            records: Some(records),
            is_padding: true,
        }
    }

    pub fn from_bytes_notemplate(data: &[u8]) -> ParseResult<DataFlow> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = take_u16(&data)?;
        let (rest, length) = take_u16(&rest)?;
        let record_len = (length - Self::HEADER_LEN) as usize;
        let record_bytes = &rest[..record_len];
        let rest = &rest[record_len..];

        Ok((
            rest,
            DataFlow {
                flowset_id,
                length,
                record_bytes: record_bytes.to_vec(),
                records: None,
                is_padding: length % 4 == 0,
                // if length is 4*n, padding exists or dataflow is aligned already.
            },
        ))
    }

    /// length is dataflow length, not record-payload's length
    fn remove_padding(length: u16, template_len: u16, payload: &[u8]) -> &[u8] {
        let padding = Self::get_padding_size(length, template_len);

        &payload[(padding as usize)..]
    }

    pub fn from_bytes<'a, T>(data: &'a [u8], templates: &'a [T]) -> ParseResult<'a, DataFlow>
    where
        T: TemplateParser,
    {
        debug!("Length of parsing data: {}", data.len());
        let (rest, flowset_id) = take_u16(&data)?;
        let (rest, length) = take_u16(&rest)?;

        let match_template: Vec<&T> = templates
            .iter()
            .filter(|template| template.get_id() == flowset_id)
            .collect();

        // can use pattern-matching? research list match
        if !match_template.is_empty() {
            let template = match_template[0];
            let bytes = rest;
            let (rest, records) = template.parse_dataflows(length - Self::HEADER_LEN, rest)?;
            let rest_not_padding = Self::remove_padding(length, template.get_template_len(), rest);
            // if padding was removed or dataflow len is aligned by 4, padding exists.
            let is_padding = rest_not_padding.len() != rest.len() || length % 4 == 0;

            Ok((
                rest_not_padding,
                DataFlow {
                    flowset_id,
                    length,
                    record_bytes: bytes[..(length as usize - 4)].to_vec(),
                    records: Some(records),
                    is_padding,
                },
            ))
        } else {
            Err(Error::TemplateNotFound)
        }
    }

    fn get_padding_size(payload_len: u16, template_len: u16) -> u16 {
        (payload_len - Self::HEADER_LEN) % template_len
    }

    pub fn is_padding(&self) -> bool {
        self.is_padding
    }

    pub fn set_padding(&mut self, is_padding: bool) {
        self.is_padding = is_padding;
    }

    /// WARN: cannot remove padding without template data
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];

        u16_to_bytes(self.flowset_id, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        // WARN: cannot use letgth field because of mutation
        // u16_to_bytes(self.byte_length() as u16, &mut u16_buf);
        // u16_to_bytes(self.length, &mut u16_buf);
        // bytes.append(&mut u16_buf.to_vec());

        // should make records to bytes to check correct dataflow length
        let mut record_bytes = match &self.records {
            Some(records) => {
                let mut record_bytes = Vec::new();

                for record in records {
                    record_bytes.append(&mut record.to_bytes());
                }

                record_bytes
            }
            None => self.record_bytes.to_vec(),
        };

        // TODO: remove padding byte. But cannot remove padding without template info
        // check padding exists or not
        let padding = if self.is_padding() {
            let padding_len = (4 - record_bytes.len() % 4) % 4;
            vec![0; padding_len]
        } else {
            vec![]
        };

        record_bytes.extend(padding);

        // set bytes ordered by length, records, padding
        u16_to_bytes(record_bytes.len() as u16 + Self::HEADER_LEN, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        bytes.extend(record_bytes);

        bytes
    }

    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }
}

#[cfg(test)]
mod test_data_flow {
    use super::DataFlow;
    use flowset::{test_data, DataTemplate};

    #[test]
    fn from_bytes_notemplate() {
        let packet_bytes = test_data::DATAFLOW_DATA;
        let res = DataFlow::from_bytes_notemplate(&packet_bytes);

        assert!(res.is_ok());
        // TODO: add field value test
    }

    #[test]
    fn from_bytes() {
        let (test_template, testdata) = test_data::TEMPLATE_AND_DATA;
        let template = DataTemplate::from_bytes(&test_template);
        assert!(template.is_ok());
        let template: DataTemplate = template.unwrap().1;

        let dataflow = DataFlow::from_bytes(&testdata, &template.templates);
        assert!(dataflow.is_ok());
        let dataflow: DataFlow = dataflow.unwrap().1;

        assert!(dataflow.records.is_some());
        assert_eq!(dataflow.flowset_id, 1024);
        assert_eq!(dataflow.length, 484);

        let records = dataflow.records.unwrap();
        assert_eq!(records.len(), 8);
    }

    // TODO: think more suitable test name
    #[test]
    fn to_bytes() {
        let (test_template, testdata) = test_data::TEMPLATE_AND_DATA;
        let template = DataTemplate::from_bytes(&test_template).unwrap().1;
        let mut dataflow = DataFlow::from_bytes(&testdata, &template.templates)
            .unwrap()
            .1;

        let bytes = dataflow.to_bytes();
        assert_eq!(bytes.len() % 4, 0);
        assert_eq!(&bytes.as_slice(), &testdata.as_ref());

        dataflow.set_padding(false);
        let bytes = dataflow.to_bytes();
        assert_eq!(&bytes.as_slice(), &testdata.as_ref());
    }

    #[test]
    fn to_bytes_no_padding() {
        let (test_template, (no_padding_data, padding_data)) =
            test_data::TEMPLATE_AND_DATA_WITH_PADDING;
        let template = DataTemplate::from_bytes(&test_template).unwrap().1;

        // no padding
        let mut dataflow = DataFlow::from_bytes(&no_padding_data, &template.templates)
            .unwrap()
            .1;

        // if padding doesn't exist, is_padding is false
        assert!(!dataflow.is_padding(), "is_padding is true");
        let bytes = dataflow.to_bytes();
        assert_eq!(
            &bytes.as_slice(),
            &no_padding_data.as_ref(),
            "Wrong when no padding"
        );

        // set padding flag
        dataflow.set_padding(true);

        let bytes = dataflow.to_bytes();
        let mut padding = Vec::new();
        padding.extend_from_slice(&padding_data[..]);

        assert_eq!(bytes.len() % 4, 0, "Not aligned");
        assert_eq!(&bytes, &padding, "Wrong when padding, is_padding true");
    }

    #[test]
    fn to_bytes_padding() {
        let (test_template, (no_padding_data, padding_data)) =
            test_data::TEMPLATE_AND_DATA_WITH_PADDING;
        let template = DataTemplate::from_bytes(&test_template).unwrap().1;

        let mut dataflow = DataFlow::from_bytes(&padding_data, &template.templates)
            .unwrap()
            .1;
        assert!(dataflow.is_padding(), "is_padding is false");

        let bytes = dataflow.to_bytes();
        assert_eq!(bytes.len() % 4, 0, "Not aligned");
        assert_eq!(
            &bytes.as_slice(),
            &padding_data.as_ref(),
            "Wrong when padding, is_padding true"
        );

        // set padding flag, but already aligned
        dataflow.set_padding(false);

        let bytes = dataflow.to_bytes();
        let mut no_padding: Vec<u8> = Vec::new();
        no_padding.extend(&no_padding_data[..]);
        assert_eq!(&bytes, &no_padding, "Wrong when padding, is_padding false");
    }

    #[test]
    fn convert_from_to() {
        let (test_template, testdata) = test_data::TEMPLATE_AND_DATA;
        let template = DataTemplate::from_bytes(&test_template).unwrap().1;
        let dataflow = DataFlow::from_bytes(&testdata, &template.templates)
            .unwrap()
            .1;
        let bytes = dataflow.to_bytes();

        let dataflow = DataFlow::from_bytes(&bytes, &template.templates).unwrap().1;
        let re_bytes = dataflow.to_bytes();

        assert_eq!(re_bytes, bytes);
    }

    #[test]
    fn byte_length() {
        let (test_template, testdata) = test_data::TEMPLATE_AND_DATA;
        // Test data's length is aligned by 4, this isn't suitable for test.
        let template = DataTemplate::from_bytes(&test_template).unwrap().1;
        let mut dataflow = DataFlow::from_bytes(&testdata, &template.templates)
            .unwrap()
            .1;

        assert_eq!(dataflow.byte_length(), testdata.len());

        dataflow.set_padding(false);
        assert_eq!(dataflow.byte_length(), testdata.len());
    }

    #[test]
    fn get_padding_size() {
        let template_len = 12;
        let length = 39 + 4;
        let pad_len = DataFlow::get_padding_size(length, template_len);

        assert_eq!(pad_len, 3);
    }

    #[test]
    fn test_remove_padding() {
        let payload = [0; 39];
        let template_len = 12;
        let length = 39 + 4;

        let payload = DataFlow::remove_padding(length, template_len, &payload);

        assert_eq!(payload.len(), 36);
    }
}
