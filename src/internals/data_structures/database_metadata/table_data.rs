use std::collections::HashMap;

use tiberius::time::chrono::{self, DateTime, NaiveDateTime};

#[derive(Debug)]
pub enum GenericData {
    Text(Option<String>),
    SmallInt(Option<i8>),
    Int(Option<i32>),
    Float(Option<f64>),
    Bool(Option<bool>),
    Bit(Option<u8>),
    DateTimeLocal(Option<NaiveDateTime>),
    DateTimeOffset(Option<DateTime<chrono::Utc>>),
}

#[derive(Debug)]
pub struct CanonnicalColumns {
    table_name: String,
    values: HashMap<String /*column_name*/, Vec<GenericData>>, /*held value*/
}

impl CanonnicalColumns {
    pub fn new(table_name: String, col_name: String, value: Vec<GenericData>) -> Self {
        Self {
            table_name,
            values: HashMap::from([(col_name, value)]),
        }
    }
}
