use std::collections::HashMap;

use crate::internals::data_structures::{
    database_metadata::db_metadata::{
        cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata,
    },
    database_types::types::TypeMapper,
};

fn build_columns_deprc(column: &ColumnMembers, types_conversion: &Vec<&TypeMapper>) -> Option<String> {
    let mut ddl_column = String::new();
    let backup = &&TypeMapper::empty_struct();
    let field_type: &str = types_conversion
        .iter()
        .find(|pred| {
            pred.get_type_origin()
                .eq_ignore_ascii_case(column.get_data_type())
        })
        .unwrap_or(backup)
        .get_type_destiny();
    ddl_column.push_str("\"");
    ddl_column.push_str(&column.get_column_name().replace(" ", "_"));
    ddl_column.push_str("\"");
    ddl_column.push_str(" ");
    ddl_column.push_str(field_type);
    if *column.get_lenght_field() > 0 {
        ddl_column.push_str("(");
        ddl_column.push_str(&column.get_lenght_field().to_string());
        ddl_column.push_str(")")
    }
    //precision read
    if *column.get_numeric_precision() > 0
        && *column.get_numeric_scale() > 0
        && types_conversion
            .iter()
            .any(|pred| pred.get_type_origin().eq(field_type))
    {
        println!("Anyways is always True ?");
        ddl_column.push_str("(");
        ddl_column.push_str(&column.get_numeric_precision().to_string());
        ddl_column.push_str(",");
        ddl_column.push_str(&column.get_numeric_scale().to_string());
        ddl_column.push_str(")")
    }
    if !column.get_is_nullable() {
        ddl_column.push_str(" NOT NULL");
    }
    Some(ddl_column)
}

fn build_constraints() {}

fn build_pks() {}

pub fn translate_ddl(
    structs_table: &HashMap<(String, String), TableMetadata>,
    types_conversion: Vec<&TypeMapper>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut ddl_content: Vec<String> = Vec::new();
    for struct_tb in structs_table {
        let mut ddl_generation = String::new();
        
    }
    
    // fields_table.iter().for_each(|(key, value)| {
    //     let mut ddl_table = String::new();
    //     let col_schema_name = value
    //         .iter()
    //         .find(|pred| pred.get_table_name().eq_ignore_ascii_case(key))
    //         .unwrap()
    //         .get_table_schema()
    //         .to_ascii_lowercase();
    //     if col_schema_name.is_empty() {
    //         ddl_table.push_str(&format!("create table public.\"{}\" (", key));
    //     } else {
    //         ddl_table.push_str(&format!(
    //             "create table \"{}\".\"{}\" (",
    //             col_schema_name, key
    //         ));
    //     }
    //     let field_ddl = value
    //         .iter()
    //         .map(|data| build_columns(data, &types_conversion).unwrap_or("".to_string()))
    //         .collect::<Vec<String>>()
    //         .join(",");
    //     ddl_table.push_str(&field_ddl);
    //     ddl_table.push_str(" );");
    //     ddl_content.push(ddl_table);
    // });
    Ok(ddl_content)
}
