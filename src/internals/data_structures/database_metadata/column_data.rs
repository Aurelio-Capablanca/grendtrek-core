#[derive(Debug, Default)]
pub struct ColumnMembers {
    column_name: Option<String>,
    data_type: Option<String>,
    length_field: Option<i32>,
    description: Option<String>,
    constraint_name: Option<String>,
    constraint_type: Option<String>,
    is_nullable: Option<bool>,
    table_name: Option<String>,
    table_schema: Option<String>,
    numeric_precision: Option<i32>,
    numeric_scale: Option<i32>,
}

impl ColumnMembers {
    pub fn new(
        column_name: Option<String>,
        data_type: Option<String>,
        length_field: Option<i32>,
        description: Option<String>,
        constraint_name: Option<String>,
        constraint_type: Option<String>,
        is_nullable: Option<bool>,
        table_name: Option<String>,
        table_schema: Option<String>,
        numeric_precision: Option<i32>,
        numeric_scale: Option<i32>,
    ) -> Self {
        Self {
            column_name: column_name,
            data_type: data_type,
            length_field: length_field,
            description: description,
            constraint_name: constraint_name,
            constraint_type: constraint_type,
            is_nullable: is_nullable,
            table_name: table_name,
            table_schema: table_schema,
            numeric_precision: numeric_precision,
            numeric_scale: numeric_scale,
        }
    }

    pub fn empty_structure() -> Self {
        Self {
            column_name: Some("".to_string()),
            data_type: Some("".to_string()),
            length_field: Some(0),
            description: Some("".to_string()),
            constraint_name: Some("".to_string()),
            constraint_type: Some("".to_string()),
            is_nullable: Some(true),
            table_name: Some("".to_string()),
            table_schema: Some("".to_string()),
            numeric_precision: Some(0),
            numeric_scale: Some(0),
        }
    }

    pub fn to_string_conversion(&self) -> String {
        format!(
            "ColumnMember:  {},{},{},{},{},{},{},{},{},{},{}",
            self.get_column_name(),
            self.get_data_type(),
            self.get_lenght_field(),
            self._get_description(),
            self.get_constraint_name(),
            self.get_constraint_type(),
            self.get_is_nullable(),
            self.get_table_name(),
            self.get_table_schema(),
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

    pub fn get_table_schema(&self) -> &str {
        self.table_schema.as_deref().unwrap_or("")
    }

    pub fn get_is_nullable(&self) -> bool {
        self.is_nullable.unwrap_or(false)
    }

    pub fn get_constraint_type(&self) -> &str {
        self.constraint_type.as_deref().unwrap_or("")
    }

    pub fn get_constraint_name(&self) -> &str {
        self.constraint_name.as_deref().unwrap_or("")
    }

    pub fn _get_description(&self) -> &str {
        self.description.as_deref().unwrap_or("")
    }

    pub fn get_lenght_field(&self) -> i32 {
        self.length_field.unwrap_or(0)
    }

    pub fn get_data_type(&self) -> &str {
        self.data_type.as_deref().unwrap_or("")
    }

    pub fn get_table_name(&self) -> &str {
        self.table_name.as_deref().unwrap_or("")
    }

    pub fn get_column_name(&self) -> &str {
        self.column_name.as_deref().unwrap_or("")
    }
}
