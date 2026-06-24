use std::collections::HashMap;

use bb8_tiberius::ConnectionManager;
use tiberius::Row;

use crate::internals::data_structures::database_metadata::{
    constraint_metadata::{
        IdentitySpecification,
        SQLConstraints::{self, PRIMARYKEY},
    },
    db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    table_data::CanonnicalColumns,
};

fn rows_to_canonnical(row: &Row) -> Result<Vec<CanonnicalColumns>, Box<dyn std::error::Error>> {
    let data_columns: Vec<CanonnicalColumns> = Vec::new();
    for (i, column) in row.columns().iter().enumerate() {
        let col_name = column.name();
        println!(
            "Column Name : {:?} | Column Type : {:?}",
            col_name,
            column.column_type()
        );
    }
    Ok(Vec::new())
}

fn query_builder(columns: &Vec<ColumnMembers>) -> String {
    columns
        .iter()
        .map(|col| {
            let col_name = col.get_column_name();
            println!("Current Column: {:?}", col_name);
            if col.get_data_type().eq_ignore_ascii_case("hierarchyid") {
                format!("CAST([{}] as VARCHAR) as [{}]", col_name, col_name)
            } else if col.get_data_type().eq_ignore_ascii_case("xml")
                || col.get_data_type().eq_ignore_ascii_case("geography")
                || col.get_column_name().eq_ignore_ascii_case("other")
            {
                format!("CAST([{}] as NVARCHAR(max)) as [{}]", col_name, col_name)
            } else {
                format!("[{}]", col_name)
            }
        })
        .collect::<Vec<String>>()
        .join(" , ")
}

pub async fn get_rows_from_tables(
    tables_metadata: &HashMap<(String, String), TableMetadata>,
    connection: &mut bb8::PooledConnection<'_, ConnectionManager>,
) -> Result<bool, Box<dyn std::error::Error>> {
    for metadata in tables_metadata {
        let table_key: &(String, String) = metadata.0;
        let table_metadata: &TableMetadata = metadata.1;
        println!("Reference columns : {:?}", table_metadata.get_cols_as_ref());
        let empty_otherwise = &SQLConstraints::PRIMARYKEY(IdentitySpecification::empty_struct());
        let pk_identifier = table_metadata
            .get_constrs_as_ref()
            .iter()
            .find(|pred| match pred {
                SQLConstraints::PRIMARYKEY(_) => true,
                _ => false,
            })
            .unwrap_or(empty_otherwise);
        let columns_query = query_builder(table_metadata.get_cols_as_ref());
        let query_build = format!(
            "SELECT {} FROM [{}].[{}] ORDER BY [{}]  OFFSET 0 ROWS FETCH NEXT 10 ROWS ONLY;",
            columns_query,
            table_key.1,
            table_key.0,
            pk_identifier
                .get_pk_ref_opt()
                .unwrap()
                .get_col_name_as_ref()
        );
        println!("{:?}", query_build);
        let rows_tables = connection
            .query(query_build, &[])
            .await
            .unwrap()
            .into_first_result()
            .await
            .unwrap();
        for row in rows_tables {
            let canonical_row = rows_to_canonnical(&row);
        }
    }
    Ok(true)
}
