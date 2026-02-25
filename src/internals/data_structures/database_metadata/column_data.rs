#[derive(Debug, Default)]
pub struct ColumnMembers {
    column_name: Option<String>,
    data_type: Option<String>,
    length_field: Option<i32>,
    is_nullable: Option<bool>,    
    numeric_precision: Option<i32>,
    numeric_scale: Option<i32>,
}

impl ColumnMembers {
    pub fn new(
        column_name: Option<String>,
        data_type: Option<String>,
        length_field: Option<i32>,        
        is_nullable: Option<bool>,
        numeric_precision: Option<i32>,
        numeric_scale: Option<i32>,
    ) -> Self {
        Self {
            column_name: column_name,
            data_type: data_type,
            length_field: length_field,            
            is_nullable: is_nullable,
            numeric_precision: numeric_precision,
            numeric_scale: numeric_scale,
        }
    }

    pub fn empty_structure() -> Self {
        Self {
            column_name: Some("".to_string()),
            data_type: Some("".to_string()),
            length_field: Some(0),            
            is_nullable: Some(true),
            numeric_precision: Some(0),
            numeric_scale: Some(0),
        }
    }

    pub fn to_string_conversion(&self) -> String {
        format!(
            "ColumnMember:  {},{},{},{},{},{}",
            self.get_column_name(),
            self.get_data_type(),
            self.get_lenght_field(),     
            self.get_is_nullable(),
            self.get_numeric_precision(),
            self.get_numeric_scale()
        )
    }

    pub fn get_numeric_scale(&self) -> i32 {
        self.numeric_scale.unwrap_or(0)
    }

    pub fn get_numeric_precision(&self) -> i32 {
        self.numeric_precision.unwrap_or(0)
    }    

    pub fn get_is_nullable(&self) -> bool {
        self.is_nullable.unwrap_or(false)
    }

    pub fn get_lenght_field(&self) -> i32 {
        self.length_field.unwrap_or(0)
    }

    pub fn get_data_type(&self) -> &str {
        self.data_type.as_deref().unwrap_or("")
    }

    pub fn get_column_name(&self) -> &str {
        self.column_name.as_deref().unwrap_or("")
    }
}
