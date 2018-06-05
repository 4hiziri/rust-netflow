#[cfg(test)]
mod flowset_tests;
#[cfg(test)]
mod test_data;

mod data_template;
pub use self::data_template::*;

mod option_template;
use self::option_template::*;

mod data_flow;
use self::data_flow::*;

mod template;
use self::template::*;

use util::take_u16;

#[derive(Debug)]
pub enum FlowSet {
    DataTemplate(DataTemplate),
    OptionTemplate(OptionTemplate),
    DataFlow(DataFlow),
}

impl FlowSet {
    // TODO: parse with template
    pub fn from_bytes(data: &[u8]) -> Result<(&[u8], Self), ()> {
        let (_, id) = take_u16(&data).unwrap();
        let id = id.unwrap().1;

        info!("parsed flowset id: {:?}", id);

        match id {
            TEMPLATE_FLOWSET_ID => {
                // Err
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

    // TODO:
    pub fn to_vec(data: &[u8]) -> Result<(&[u8], Vec<Self>), ()> {
        let mut rest = data;

        while rest.len() > 0 {
            let (next, flowset) = FlowSet::from_bytes(&data).unwrap();

            match flowset {
                FlowSet::DataTemplate(template) => {}
                FlowSet::OptionTemplate(template) => {}
                FlowSet::DataFlow(template) => {}
            }

            rest = next;
        }

        Err(())
    }
}
