#[allow(non_upper_case_globals)]
#[allow(unused)]
mod field_types;
#[allow(non_snake_case)]
pub mod FieldTypes {
    #[allow(unused_imports)]
    use crate::field::field_types::*;
}

#[allow(non_upper_case_globals)]
#[allow(unused)]
mod scope_types;
#[allow(non_snake_case)]
pub mod ScopeTypes {
    #[allow(unused_imports)]
    use crate::field::scope_types::*;
}

mod field_value;
pub use self::field_value::*;

mod mac_addr;
pub use self::mac_addr::*;

mod type_length_field;
pub use self::type_length_field::*;

#[cfg(test)]
mod test_data;

use crate::error::{Error, ParseResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowField {
    type_id: u16,
    length: u16,
    value: FieldValue,
}

impl FlowField {
    pub fn new(type_id: u16, length: u16, value: FieldValue) -> FlowField {
        FlowField {
            type_id,
            length,
            value,
        }
    }

    pub fn from_bytes(type_id: u16, length: u16, bytes: &[u8]) -> ParseResult<FlowField> {
        if (length as usize) <= bytes.len() {
            Ok((
                &bytes[(length as usize)..],
                FlowField::new(
                    type_id,
                    length,
                    FieldValue::new(type_id, &bytes[..(length as usize)]),
                ),
            ))
        } else {
            Err(Error::InvalidLength)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_bytes(self.length)
    }

    pub fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }
}

// TODO: add test
