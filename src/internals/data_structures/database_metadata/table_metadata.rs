use crate::internals::data_structures::database_metadata::column_data::ColumnMembers;

pub struct TableMetadata{
    table_name : String,
    table_schema: String,
    columns: Vec<ColumnMembers>,
}