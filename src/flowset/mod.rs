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

mod data_template_item;
use self::data_template_item::*;

mod option_template_item;
use self::option_template_item::*;

mod record;
use self::record::*;

mod template_parser;
use self::template_parser::*;

use error::ParseResult;
use util::take_u16;

#[derive(Debug)]
pub enum FlowSet {
    DataTemplate(DataTemplate),
    OptionTemplate(OptionTemplate),
    DataFlow(DataFlow),
}

impl FlowSet {
    // TODO: parse with template
    pub fn from_bytes(data: &[u8]) -> ParseResult<FlowSet> {
        let (_, id) = take_u16(&data).unwrap(); // num::IResult

        info!("parsed flowset id: {:?}", id);

        match id {
            TEMPLATE_FLOWSET_ID => {
                let (next, template) = DataTemplate::from_bytes(&data).unwrap(); // Err
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

    pub fn to_vec<'a>(data: &'a [u8]) -> ParseResult<'a, Vec<FlowSet>> {
        let mut rest = data;
        let mut sets: Vec<FlowSet> = Vec::new();

        while rest.len() > 0 {
            let (next, flowset) = FlowSet::from_bytes(&rest).unwrap();
            sets.push(flowset);
            rest = next;
        }

        // TODO: apply template?

        Ok((rest, sets))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            FlowSet::DataTemplate(template) => template.to_bytes(),
            FlowSet::OptionTemplate(template) => template.to_bytes(),
            FlowSet::DataFlow(dataflow) => dataflow.to_bytes(),
        }
    }
}

#[cfg(test)]
mod test_flowset {
    use super::FlowSet;
    use flowset::test_data;

    #[test]
    fn test_frombytes() {
        let test_data = test_data::FLOWSET_DATA;
        let set = FlowSet::from_bytes(&test_data);
        assert!(set.is_ok());
        let (_rest, set) = set.unwrap();

        assert!(match set {
            FlowSet::DataTemplate(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_to_vec() {
        let test_data = test_data::MULTI_FLOWSET_DATA;
        let sets = FlowSet::to_vec(&test_data);
        assert!(sets.is_ok());
        let (rest, sets) = sets.unwrap();

        assert_eq!(rest.len(), 0);
        assert_eq!(sets.len(), 7);

        for i in 0..4 {
            assert!(
                match sets[i] {
                    FlowSet::DataTemplate(_) => true,
                    _ => false,
                },
                "failed at {}",
                i
            );
        }

        assert!(
            match sets[4] {
                FlowSet::OptionTemplate(_) => true,
                _ => false,
            },
            "Failed at 4"
        );

        for i in 5..7 {
            assert!(
                match sets[i] {
                    FlowSet::DataFlow(_) => true,
                    _ => false,
                },
                "failed at {}",
                i
            );
        }
    }

    #[test]
    fn test_to_bytes() {
        let test_data = test_data::FLOWSET_DATA;
        let (_, set) = FlowSet::from_bytes(&test_data).unwrap();
        let bytes = set.to_bytes();

        assert_eq!(&bytes.as_slice(), &test_data.as_ref());
    }
}
