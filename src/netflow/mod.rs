#[cfg(test)]
mod test_data;
#[cfg(test)]
mod tests;

use flowset::FlowSet;
use util::{take_u16, take_u32};

// TODO: impl method struct into bytes
// TODO: impl Err

// Netflow V9 -> Header + (Template* Option* Data*)

// TODO: enum NetFlow or abstract with Netflow struct
#[derive(Debug)]
pub struct NetFlow9 {
    pub version: u16,
    pub count: u16,
    pub sys_uptime: u32, // FIXME: replace proper type like time?
    pub timestamp: u32,
    pub flow_sequence: u32,
    pub source_id: u32,
    pub flow_sets: Vec<FlowSet>,
}

impl NetFlow9 {
    fn parse_flowsets(data: &[u8]) -> Result<Vec<FlowSet>, ()> {
        let mut rest: &[u8] = data;
        let mut flowsets = Vec::<FlowSet>::new();

        while rest.len() != 0 {
            let (next, flowset) = FlowSet::from_bytes(&rest).unwrap(); // Err
            flowsets.push(flowset);
            rest = next;
        }

        Ok(flowsets)
    }

    pub fn from_bytes(payload: &[u8]) -> Result<Self, ()> {
        let (payload, version) = take_u16(payload).unwrap(); // Err
        let version = version.unwrap().1;

        if version == 9 {
            let (payload, count) = take_u16(payload).unwrap();
            let (payload, sys_uptime) = take_u32(payload).unwrap();
            let (payload, timestamp) = take_u32(payload).unwrap();
            let (payload, flow_sequence) = take_u32(payload).unwrap();
            let (payload, source_id) = take_u32(payload).unwrap();
            let flow_sets = NetFlow9::parse_flowsets(payload).unwrap(); // Err

            Ok(NetFlow9 {
                version: version,
                count: count.unwrap().1, // Err
                sys_uptime: sys_uptime.unwrap().1,
                timestamp: timestamp.unwrap().1,
                flow_sequence: flow_sequence.unwrap().1,
                source_id: source_id.unwrap().1,
                flow_sets: flow_sets,
            })
        } else {
            Err(())
        }
    }
}
