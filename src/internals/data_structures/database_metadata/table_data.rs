use std::collections::HashMap;

use tiberius::time::chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub enum GenericDataSQLServer {
    Text(Option<String>),
    SmallInt(Option<i16>),
    Int(Option<i32>),
    Float(Option<f64>),
    Bool(Option<bool>),
    Bit(Option<u8>),
    DateTimeLocal(Option<NaiveDateTime>),
    Date(Option<NaiveDate>),
    BigBinary(Option<Vec<u8>>),
}

#[derive(Debug)]
pub enum GenericDatasetDBMS {
    PG,
    SQLSERVER(GenericDataSQLServer),
}

#[derive(Debug)]
pub struct CanonnicalColumns {
    table_name: String,
    values: HashMap<String /*column_name*/, Vec<GenericDatasetDBMS>>, /*held value*/
}

impl CanonnicalColumns {
    pub fn new(table_name: String, cols: HashMap<String, Vec<GenericDatasetDBMS>>) -> Self {
        Self {
            table_name,
            values: cols,
        }
    }

    pub fn new_all_in(
        table_name: String,
        col_name: String,
        value: Vec<GenericDatasetDBMS>,
    ) -> Self {
        Self {
            table_name,
            values: HashMap::from([(col_name, value)]),
        }
    }
}
