use serde::Deserialize;

use crate::internals::data_structures::database_connector_spec::VendorOptions;

#[derive(Debug, Deserialize)]
pub struct Query {
    engine: VendorOptions,
    id: i32,
    query: String,
    note: String,
}

impl Query {
        
    pub fn empty_query() -> Self {
        Self { engine: VendorOptions::NONE, id: 0, query: String::new(), note: String::new() }
    }
    
    pub fn engine_out(&self) -> &VendorOptions{
        &self.engine
    }
    
    pub fn id_out(&self) -> &i32{
        &self.id
    }
    
    pub fn query_out(&self)-> &str{
        &self.query
    }
    
    
}
