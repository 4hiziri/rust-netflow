#[cfg(test)]
mod flowset_tests;
#[cfg(test)]
mod test_data;

mod data_template;
use self::data_template::*;

mod option_template;
use self::option_template::*;

mod data_flow;
use self::data_flow::*;

use nom;
use nom::be_u16;

#[derive(Debug)]
pub enum FlowSet {
    DataTemplate(DataTemplate),
    OptionTemplate(OptionTemplate),
    DataFlow(DataFlow),
}

named!(flowset_id <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(flowset_length <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(template_id <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));

impl FlowSet {
    // TODO: parse with template
    pub fn from_bytes(data: &[u8]) -> Result<(&[u8], FlowSet), ()> {
        let (_, id) = flowset_id(&data).unwrap();
        let id = id.unwrap().1;
        info!("parsed flowset id: {:?}", id);

        match id {
            TEMPLATE_FLOWSET_ID => {
                let (next, template) = DataTemplate::from_bytes(&data).unwrap(); // TODO: use combinator
                debug!("parsed DataTemplate: {:?}", template);
                Ok((next, FlowSet::DataTemplate(template)))
            }
            OPTION_FLOWSET_ID => {
                let (next, option) = OptionTemplate::from_bytes(&data).unwrap();
                debug!("parsed OptionTemplate: {:?}", option);
                Ok((next, FlowSet::OptionTemplate(option)))
            }
            _ => {
                let (next, flow) = DataFlow::from_bytes_notemplate(&data).unwrap();
                debug!("parsed DataFlow: {:?}", flow);
                Ok((next, FlowSet::DataFlow(flow)))
            }
        }
    }
}
