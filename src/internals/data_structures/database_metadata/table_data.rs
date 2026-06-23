use std::collections::HashMap;

#[derive(Debug)]
pub enum GenericData {
    Text(Option<String>),
    SmallInt(Option<i8>),
    Int(Option<i32>),
    BigInt(Option<i32>),
    Float(Option<f32>),
    BigFloat(Option<f64>),
    Bool(Option<bool>),
    Bit(Option<u8>),
}

#[derive(Debug)]
pub struct CanonnicalColumns {
    table_name: String,
    values: HashMap<String, Vec<GenericData>>,
}

impl CanonnicalColumns {
    pub fn new(table_name: String, col_name: String, value: Vec<GenericData>) -> Self {
        Self {
            table_name,
            values: HashMap::from([(col_name, value)]),
        }
    }
}
