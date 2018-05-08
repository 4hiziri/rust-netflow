#![allow(dead_code)]

use field::{Field, Option};
use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct DataTemplate {
    flowset_id: u16,
    length: u16,
    template_id: u16,
    field_count: u16,
    fields: Vec<Field>,
}

impl DataTemplate {
    pub fn new() -> DataTemplate {
        DataTemplate {
            flowset_id: 0,
            length: 0,
            template_id: 0,
            field_count: 0,
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionTemplate {
    flowset_id: u16,
    length: u16,
    template_id: u16,
    option_scope_length: u16,
    option_length: u16,
    options: Vec<Option>,
}

#[derive(Debug, Clone)]
pub struct DataFlow {
    flowset_id: u16,
    length: u16,
    records: Vec<u16>,
}

pub trait FlowSet {}

// TODO: abstract with Netflow struct
pub struct Netflow9 {
    version: u16,
    count: u16,
    sys_up_time: u32,
    timestamp: u32,
    flow_sequence: u32,
    flowset_id: u32,
    flow_sets: Vec<u8>,
}
