use field::FlowField;
use super::{flowset_id, flowset_length, DataTemplate};

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
    pub fn from_bytes_notemplate(data: &[u8]) -> Result<(&[u8], DataFlow), ()> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let (rest, length) = flowset_length(&rest).unwrap();
        let length = length.unwrap().1;
        let record_bytes = &rest[..(length as usize - 4)];
        let rest = &rest[(length as usize - 4)..];

        Ok((
            rest,
            DataFlow::new(
                flowset_id.unwrap().1,
                length,
                Some(record_bytes.to_vec()),
                None,
            ),
        ))
    }

    fn get_template(flowset_id: u16, templates: &[DataTemplate]) -> Option<&DataTemplate> {
        let template: Vec<&DataTemplate> = templates
            .iter()
            .filter(|temp| temp.template_id == flowset_id)
            .collect();

        if template.len() == 0 {
            None
        } else {
            Some(&template[0])
        }
    }

    pub fn from_bytes<'a>(
        data: &'a [u8],
        templates: &[DataTemplate],
    ) -> Result<(&'a [u8], DataFlow), ()> {
        debug!("Length of parsing data: {}", data.len());

        let (rest, flowset_id) = flowset_id(&data).unwrap();
        let flowset_id = flowset_id.unwrap().1;
        let (rest, length) = flowset_length(&rest).unwrap();
        let length = length.unwrap().1;

        // TODO: need field parser for skipping padding
        let template: Option<&DataTemplate> = DataFlow::get_template(flowset_id, templates);

        match template {
            Some(template) => {
                let records_num = DataFlow::get_record_num(length, template.get_dataflow_length());
                let mut records: Vec<Vec<FlowField>> = Vec::with_capacity(records_num);
                let mut rest = rest;

                for _ in 0..records_num {
                    let (next, fields) = template.parse_dataflow(rest).unwrap();
                    records.push(fields);
                    rest = next;
                }

                let padding = DataFlow::get_padding(length, template.get_dataflow_length());

                if padding > 0 {
                    rest = &rest[(padding as usize)..]
                }

                // Left bytes for future parsing?
                Ok((
                    rest,
                    DataFlow::new(flowset_id, length, None, Some(records)),
                ))
            }
            None => {
                // Return Err, None or bytes field?
                debug!("Template is not found, flowset_id = {}", flowset_id);
                Err(())
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
