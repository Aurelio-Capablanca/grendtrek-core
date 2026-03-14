use std::collections::HashMap;

use crate::internals::data_structures::{
    database_metadata::{
        constraint_metadata::SQLConstraints,
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    },
    database_types::types::TypeMapper,
};

fn build_columns(column: &ColumnMembers, types_conversion: &Vec<&TypeMapper>) -> Option<String> {
    let mut column_ddl = String::new();
    column_ddl.push_str("\"");
    column_ddl.push_str(column.get_column_name());
    column_ddl.push_str("\"");
    column_ddl.push_str(" ");
    column_ddl.push_str(column.get_data_type());
    if column.get_lenght_field() > &0 {
        column_ddl.push_str("(");
        column_ddl.push_str(&column.get_lenght_field().to_string());
        column_ddl.push_str(") ");
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
        column_ddl.push_str("COLLATE \"");
        column_ddl.push_str(column.get_collation());
        column_ddl.push_str("\"  ");
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
    structs_table.iter().for_each(|struct_table| {
        let mut ddl_generation = String::new();
        let table_keys: &(String, String) = struct_table.0;
        let table_metadata: &TableMetadata = struct_table.1;
        let columns = table_metadata.get_cols_as_ref();
        let constraints = table_metadata.get_constrs_as_ref();
        ddl_generation.push_str("create table \"");
        ddl_generation.push_str(&table_keys.1);
        ddl_generation.push_str("\".\"");
        ddl_generation.push_str(&table_keys.0);
        ddl_generation.push_str("\" (");
        let field_spec = columns
            .iter()
            .map(|column| {
                if constraints.iter().any(|pred| match pred {
                    SQLConstraints::PRIMARYKEY(pk) => {
                        pk.get_col_name_as_ref().eq(column.get_column_name())
                    }
                    _ => false,
                }) {
                    //PK column
                    /*Use Serial as Type instead of the numercal given one! */
                    "PK_NON_Stated_Val".to_string()
                } else {
                    // regular column!
                    match build_columns(column, &types_conversion) {
                        Some(val) => val,
                        None => "".to_string(),
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(",");
        ddl_generation.push_str(&field_spec);
        ddl_generation.push_str(");");
        ddl_content.push(ddl_generation);
    });

    Ok(ddl_content)
}
