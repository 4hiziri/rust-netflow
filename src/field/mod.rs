#[allow(non_upper_case_globals)]
#[allow(unused)]
mod field_types;
#[allow(non_snake_case)]
pub mod FieldTypes {
    #[allow(unused_imports)]
    use field::field_types::*;
}

#[allow(non_upper_case_globals)]
#[allow(unused)]
mod scope_types;
#[allow(non_snake_case)]
pub mod ScopeTypes {
    #[allow(unused_imports)]
    use field::scope_types::*;
}

mod mac_addr;
mod field_value;
mod type_length_field;
pub use self::mac_addr::*;
pub use self::field_value::*;
pub use self::type_length_field::*;

#[derive(Debug, Clone)]
pub struct FlowField {
    type_id: u16,
    length: u16,
    value: FieldValue,
}

impl FlowField {
    pub fn new(type_id: u16, length: u16, value: FieldValue) -> FlowField {
        FlowField {
            type_id: type_id,
            length: length,
            value: value,
        }
    }

    pub fn from_bytes(type_id: u16, length: u16, bytes: &[u8]) -> Result<(&[u8], FlowField), ()> {
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
            Err(())
        }
    }
}
