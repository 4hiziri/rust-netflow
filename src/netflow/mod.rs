#[cfg(test)]
mod test_data;
#[cfg(test)]
mod tests;

use error::Error;
use flowset::FlowSet;
use util::{take_u16, take_u32};

// TODO: impl method struct into bytes

// Netflow V9 -> Header + (Template* Option* Data*)

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
}

#[cfg(test)]
mod test_netflow {
    use super::test_data;
    use netflow::*;

    // TODO: can use macro?
    fn is_template(flowset: &FlowSet) -> bool {
        match flowset {
            &FlowSet::DataTemplate(_) => true,
            _ => false,
        }
    }

    fn is_option(flowset: &FlowSet) -> bool {
        match flowset {
            &FlowSet::OptionTemplate(_) => true,
            _ => false,
        }
    }

    fn is_dataflow(flowset: &FlowSet) -> bool {
        match flowset {
            &FlowSet::DataFlow(_) => true,
            _ => false,
        }
    }

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
        assert!(is_template(&netflow.flow_sets[0]));
        assert!(is_template(&netflow.flow_sets[1]));
        assert!(is_template(&netflow.flow_sets[2]));
        assert!(is_template(&netflow.flow_sets[3]));
        assert!(is_option(&netflow.flow_sets[4]));
        assert!(is_dataflow(&netflow.flow_sets[5]));
        assert!(is_dataflow(&netflow.flow_sets[6]));
    }
}
