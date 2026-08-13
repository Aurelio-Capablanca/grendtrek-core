use std::{collections::HashMap, fmt::format};

use tiberius::time::chrono::{NaiveDate, NaiveDateTime, Utc};

use crate::internals::data_structures::database_metadata::table_data::{
    GenericDataSQLServer::SmallInt, GenericDatasetDBMS::SQLSERVER,
};

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
    values: HashMap<String /*column_name*/, Vec<GenericDatasetDBMS>>, /*held value*///(String, Vec<GenericDatasetDBMS>)//
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

    pub fn get_table_ref(&self) -> &str {
        &self.table_name
    }

    pub fn get_keys_ref(&self) -> Vec<&String> {
        let values = &self.values;
        values
            .keys()
            .into_iter()
            .map(|data| data)
            .collect::<Vec<&String>>()
    }

    pub fn get_ref_cols(&self) -> &HashMap<String, Vec<GenericDatasetDBMS>> {
        &self.values
    }

    pub fn get_ref_data(&self, key: String) -> &Vec<GenericDatasetDBMS> {
        &self.values.get(&key).unwrap()
    }

    pub fn get_ref_data_to_str(&self, key: String) -> String {
        let content_getter = &self.values.get(&key);
        let content = match content_getter {
            Some(thing) => thing,
            None => {
                return "".to_string();
            }
        };
        let output = content
            .iter()
            .map(|data| match data {
                SQLSERVER(dataset) => match dataset {
                    GenericDataSQLServer::Text(text) => {
                        text.as_ref().unwrap_or(&String::new()).to_string()
                    }
                    GenericDataSQLServer::SmallInt(sint) => {
                        let default_value = 0_i16;
                        let value = sint.as_ref().unwrap_or(&default_value);
                        format!("{:?}", value)
                    }
                    GenericDataSQLServer::Int(int) => {
                        let default_value = 0_i32;
                        let value = int.as_ref().unwrap_or(&default_value);
                        format!("{:?}", value)
                    }
                    GenericDataSQLServer::Float(float) => {
                        let default_value = 0_f64;
                        let value = float.as_ref().unwrap_or(&default_value);
                        format!("{:?}", value)
                    }
                    GenericDataSQLServer::Bool(boolean) => {
                        format!("{:?}", boolean.unwrap_or(false))
                    }
                    GenericDataSQLServer::DateTimeLocal(dtlocal) => {
                        let default_value: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
                            .unwrap()
                            .and_hms_opt(9, 10, 11)
                            .unwrap();
                        format!("{:?}", dtlocal.unwrap_or(default_value))
                    }
                    GenericDataSQLServer::Date(datet) => {
                        let default_value = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap();
                        format!("{:?}", datet.unwrap_or(default_value))
                    }
                    GenericDataSQLServer::Bit(bit) => {
                        format!("{:?}", bit.unwrap_or(0))
                    }
                    GenericDataSQLServer::BigBinary(bbin) => {
                        let default_bbin = vec![0_u8];
                        let value = bbin.as_ref().unwrap_or(&default_bbin);
                        format!("{:?}", value)
                    }
                },
                _ => {
                    format!("malformed!")
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        output
    }
}
