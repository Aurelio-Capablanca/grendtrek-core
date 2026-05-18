use serde::Deserialize;

use crate::internals::data_structures::database_connector_spec::VendorOptions;

#[derive(Deserialize, Debug)]
pub struct TypeMapper{
    engine_origin: VendorOptions,
    engine_destiny: VendorOptions,
    type_origin: String,
    type_destiny: String,
    is_precision: bool
}


impl TypeMapper {
    
    pub fn empty_struct()->Self{
        Self { engine_origin: VendorOptions::NONE, engine_destiny: VendorOptions::NONE, type_origin: "STRING".to_string(), type_destiny: "STRING".to_string(), is_precision: false }
    }
    
    pub fn get_origin_engine(&self) -> &VendorOptions {
        &self.engine_origin
    }
    
    pub fn get_destiny_engine(&self)-> &VendorOptions {
        &self.engine_destiny
    }
    
    pub fn get_type_origin(&self) -> &str {
        &self.type_origin.as_str()
    }
    
    pub fn get_type_destiny(&self) -> &str{
        &self.type_destiny.as_str()
    }
    
    pub fn get_is_precision(&self) -> &bool{
        &self.is_precision
    }
}