#[cfg(test)]
mod tests;
#[cfg(test)]
mod test_data;

use nom;
use nom::{be_u16, be_u32};
use flowset::FlowSet;

// TODO: impl method struct into bytes

// Netflow V9 -> Header + (Template* Option* Data*)

// FIXME: simplify parser, alias
// named!(take_as_u16 <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(netflow_version <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

named!(netflow9_count <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(netflow9_sys_uptime <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_timestamp <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_flow_sequence <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
named!(netflow9_source_id <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));

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
            let (next, flowset) = FlowSet::from_bytes(&rest).unwrap();
            flowsets.push(flowset);
            rest = next;
        }

        Ok(flowsets)
    }

    pub fn from_bytes(payload: &[u8]) -> Result<NetFlow9, ()> {
        let (payload, version) = netflow_version(payload).unwrap();
        let version = version.unwrap().1;

        if version == 9 {
            let (payload, count) = netflow9_count(payload).unwrap();
            let (payload, sys_uptime) = netflow9_sys_uptime(payload).unwrap();
            let (payload, timestamp) = netflow9_timestamp(payload).unwrap();
            let (payload, flow_sequence) = netflow9_flow_sequence(payload).unwrap();
            let (payload, source_id) = netflow9_source_id(payload).unwrap();
            let flow_sets = NetFlow9::parse_flowsets(payload).unwrap();

            Ok(NetFlow9 {
                version: version,
                count: count.unwrap().1,
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
