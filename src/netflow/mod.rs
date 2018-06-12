#[cfg(test)]
mod test_data;
#[cfg(test)]
mod tests;

use error::Error;
use flowset::FlowSet;
use util::{take_u16, take_u32, u16_to_bytes, u32_to_bytes};

// TODO: impl method struct into bytes

// Netflow V9 -> Header + (Template* Option* Data*)

// TODO: need mut?
// TODO: enum NetFlow or abstract with Netflow struct
#[derive(Debug)]
pub struct NetFlow9 {
    pub version: u16,
    pub count: u16,
    pub sys_uptime: u32, // TODO: replace proper type like time?
    pub timestamp: u32,
    pub flow_sequence: u32,
    pub source_id: u32,
    pub flow_sets: Vec<FlowSet>,
}

impl NetFlow9 {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, Error> {
        let (rest, version) = take_u16(payload).unwrap();

        if version == 9 {
            let (rest, count) = take_u16(rest).unwrap();
            let (rest, sys_uptime) = take_u32(rest).unwrap();
            let (rest, timestamp) = take_u32(rest).unwrap();
            let (rest, flow_sequence) = take_u32(rest).unwrap();
            let (rest, source_id) = take_u32(rest).unwrap();
            let (_rest, flow_sets) = FlowSet::to_vec(rest).unwrap();

            Ok(NetFlow9 {
                version: version,
                count: count,
                sys_uptime: sys_uptime,
                timestamp: timestamp,
                flow_sequence: flow_sequence,
                source_id: source_id,
                flow_sets: flow_sets,
            })
        } else {
            Err(Error::InvalidFieldValue)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];
        let mut u32_buf = [0u8; 4];

        u16_to_bytes(self.version, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u16_to_bytes(self.count, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u32_to_bytes(self.sys_uptime, &mut u32_buf);
        bytes.append(&mut u32_buf.to_vec());

        u32_to_bytes(self.timestamp, &mut u32_buf);
        bytes.append(&mut u32_buf.to_vec());

        u32_to_bytes(self.flow_sequence, &mut u32_buf);
        bytes.append(&mut u32_buf.to_vec());

        u32_to_bytes(self.source_id, &mut u32_buf);
        bytes.append(&mut u32_buf.to_vec());

        for flowset in &self.flow_sets {
            bytes.append(&mut flowset.to_bytes());
        }

        // TODO: padding

        bytes
    }
}

#[cfg(test)]
mod test_netflow {
    use super::test_data;
    use netflow::*;

    // TODO: extract as combination test
    #[test]
    fn test_netflow9() {
        let packet_bytes = &test_data::NETFLOWV9_DATA[..];
        let res = NetFlow9::from_bytes(&packet_bytes);
        assert!(res.is_ok());

        let netflow = res.unwrap();
        assert_eq!(netflow.version, 9);
        assert_eq!(netflow.count, 7);
        assert_eq!(netflow.sys_uptime, 5502099);
        assert_eq!(netflow.timestamp, 1523936618);
        assert_eq!(netflow.flow_sequence, 883);
        assert_eq!(netflow.flow_sets.len(), 7);
        assert!(netflow.flow_sets[0].is_template());
        assert!(netflow.flow_sets[1].is_template());
        assert!(netflow.flow_sets[2].is_template());
        assert!(netflow.flow_sets[3].is_template());
        assert!(netflow.flow_sets[4].is_option());
        assert!(netflow.flow_sets[5].is_dataflow());
        assert!(netflow.flow_sets[6].is_dataflow());
    }

    #[test]
    fn test_to_bytes() {
        let packet_bytes = &test_data::NETFLOWV9_DATA[..];
        let res = NetFlow9::from_bytes(&packet_bytes).unwrap();

        let bytes = res.to_bytes();
        assert_eq!(&bytes.as_slice(), &packet_bytes.as_ref());
    }
}
