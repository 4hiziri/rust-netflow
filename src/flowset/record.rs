use field::FlowField;

#[derive(Debug)]
pub enum Record {
    Data(DataRecord),
    OptionData(OptionRecord),
}

impl Record {
    pub fn make_data(fields: Vec<FlowField>) -> Record {
        Record::Data(DataRecord::new(fields))
    }

    pub fn make_option(scopes: Vec<FlowField>, options: Vec<FlowField>) -> Record {
        Record::OptionData(OptionRecord::new(scopes, options))
    }
}

// TODO: accessing method
#[derive(Debug)]
pub struct DataRecord {
    fields: Vec<FlowField>,
}

impl DataRecord {
    fn new(fields: Vec<FlowField>) -> DataRecord {
        DataRecord { fields: fields }
    }
}

#[derive(Debug)]
pub struct OptionRecord {
    scope_fields: Vec<FlowField>,
    option_fields: Vec<FlowField>,
}

impl OptionRecord {
    fn new(scopes: Vec<FlowField>, options: Vec<FlowField>) -> OptionRecord {
        OptionRecord {
            scope_fields: scopes,
            option_fields: options,
        }
    }
}
