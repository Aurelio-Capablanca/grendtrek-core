#[derive(Debug)]
pub struct Values<T>(String, T);

#[derive(Debug)]
pub struct TableData<T> {
    table_name: String,
    value: Values<T>,
}


impl <T> TableData<T> {
    
    pub fn new(table_name: String, key : String, value : T) -> Self {
        Self { table_name, value: Values(key, value) }
    }
    
}