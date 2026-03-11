use std::collections::HashMap;

use crate::internals::data_structures::{
    database_metadata::{
        constraint_metadata::SQLConstraints,
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    },
    database_types::types::TypeMapper,
};

// fn build_columns_deprc(
//     column: &ColumnMembers,
//     types_conversion: &Vec<&TypeMapper>,
// ) -> Option<String> {
//     let mut ddl_column = String::new();
//     let backup = &&TypeMapper::empty_struct();
//     let field_type: &str = types_conversion
//         .iter()
//         .find(|pred| {
//             pred.get_type_origin()
//                 .eq_ignore_ascii_case(column.get_data_type())
//         })
//         .unwrap_or(backup)
//         .get_type_destiny();
//     ddl_column.push_str("\"");
//     ddl_column.push_str(&column.get_column_name().replace(" ", "_"));
//     ddl_column.push_str("\"");
//     ddl_column.push_str(" ");
//     ddl_column.push_str(field_type);
//     if *column.get_lenght_field() > 0 {
//         ddl_column.push_str("(");
//         ddl_column.push_str(&column.get_lenght_field().to_string());
//         ddl_column.push_str(")")
//     }
//     //precision read
//     if *column.get_numeric_precision() > 0
//         && *column.get_numeric_scale() > 0
//         && types_conversion
//             .iter()
//             .any(|pred| pred.get_type_origin().eq(field_type))
//     {
//         println!("Anyways is always True ?");
//         ddl_column.push_str("(");
//         ddl_column.push_str(&column.get_numeric_precision().to_string());
//         ddl_column.push_str(",");
//         ddl_column.push_str(&column.get_numeric_scale().to_string());
//         ddl_column.push_str(")")
//     }
//     if !column.get_is_nullable() {
//         ddl_column.push_str(" NOT NULL");
//     }
//     Some(ddl_column)
// }

fn build_columns(column: &ColumnMembers, types_conversion: &Vec<&TypeMapper>) -> Option<String> {
    let mut column_ddl = String::new();
    column_ddl.push_str("\"");
    column_ddl.push_str(column.get_column_name());
    column_ddl.push_str("\"");
    column_ddl.push_str(" ");
    column_ddl.push_str(column.get_data_type());
    if column.get_lenght_field() > &0 {
        column_ddl.push_str(&column.get_lenght_field().to_string());
    }
    if column.get_numeric_precision() > &0
        && column.get_numeric_scale() > &0
        && types_conversion
            .iter()
            .any(|pred| pred.get_type_origin().eq(column.get_data_type()))
    {
        column_ddl.push_str("(");
        column_ddl.push_str(&column.get_numeric_precision().to_string());
        column_ddl.push_str(",");
        column_ddl.push_str(&column.get_numeric_scale().to_string());
        column_ddl.push_str(")");
    }
    column_ddl.push_str(" ");
    if !column.get_collation().is_empty() {
        column_ddl.push_str(column.get_collation());
        column_ddl.push_str(" ");
    }
    if *column.get_is_nullable() {
        column_ddl.push_str("NULL");        
    } else {
        column_ddl.push_str("NOT NULL");
    }
    
    Some(column_ddl)
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
        let table_keys: &(String, String) = struct_tb.0;
        let table_metadata: &TableMetadata = struct_tb.1;
        let columns = table_metadata.get_cols_as_ref();
        for (i, column) in columns.iter().enumerate() {
            if table_metadata.get_constrs().iter().any(|pred| match pred {
                SQLConstraints::PRIMARYKEY(pk) => {
                    pk.get_col_name_as_ref().eq(column.get_column_name())
                }
                _ => false,
            }) {
                //PK column
                /*Use Serial as Type instead of the numercal given one! */
            } else {
                // regular column!
                let field = match build_columns(column, &types_conversion) {
                    Some(val) => val,
                    None => "".to_string(),
                };
            }
        }
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
