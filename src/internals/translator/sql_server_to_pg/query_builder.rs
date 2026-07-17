use std::{collections::HashMap, vec};

use bb8_tiberius::ConnectionManager;
use tiberius::{
    ColumnType::{self},
    Row, Uuid,
    numeric::Numeric,
    time::chrono::{NaiveDate, NaiveDateTime},
};

use crate::internals::data_structures::database_metadata::{
    constraint_metadata::{
        IdentitySpecification,
        SQLConstraints::{self, PRIMARYKEY},
    },
    db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    table_data::{CanonnicalColumns, GenericDataSQLServer, GenericDatasetDBMS},
};

fn rows_to_canonnical(
    row: &Row,
) -> Result<HashMap<String, Vec<GenericDatasetDBMS>>, Box<dyn std::error::Error>> {
    let mut data_columns: HashMap<String, Vec<GenericDatasetDBMS>> = HashMap::new();
    for (i, column) in row.columns().iter().enumerate() {
        let col_name = column.name();
        let col_type = column.column_type();
        let value = match col_type {
            ColumnType::Int4 => GenericDataSQLServer::Int(row.get(i)),
            ColumnType::Int2 => GenericDataSQLServer::SmallInt(row.get(i)),
            ColumnType::Int1 => {
                GenericDataSQLServer::Bit(row.get::<u8, _>(i).map(|data| data.to_be()))
            }
            ColumnType::NVarchar | ColumnType::NChar => {
                GenericDataSQLServer::Text(row.get::<&str, _>(i).map(|data| data.to_string()))
            }
            ColumnType::BigVarChar => {
                GenericDataSQLServer::Text(row.get::<&str, _>(i).map(|data| data.to_string()))
            }
            ColumnType::Datetime | ColumnType::Datetimen => {
                let val: Option<NaiveDateTime> = row.get(i);
                GenericDataSQLServer::DateTimeLocal(val)
            }
            ColumnType::Daten => {
                let val: Option<NaiveDate> = row.get(i);
                GenericDataSQLServer::Date(val)
            }
            ColumnType::BigVarBin => {
                let val: Option<Vec<u8>> = row.get::<&[u8], _>(i).map(|b| b.to_vec());
                GenericDataSQLServer::BigBinary(val)
            }
            ColumnType::Numericn | ColumnType::Decimaln => {
                let decimal_n: Numeric =
                    row.get(i).unwrap_or_else(|| Numeric::new_with_scale(0, 0));
                GenericDataSQLServer::Float(Some(f64::from(decimal_n)))
            }
            ColumnType::Money => GenericDataSQLServer::Float(row.get(i)),
            ColumnType::Bit => GenericDataSQLServer::Bool(row.get(i)),
            ColumnType::Guid => {
                let unique_id: Uuid = row.get(i).unwrap();
                GenericDataSQLServer::Text(Some(unique_id.to_string()))
            }
            _ => GenericDataSQLServer::Text(Some("nd".to_string())),
        };
        /*println!(
            "Column Name : {:?} | Column Type : {:?} | Column Value : {:?}",
            col_name, col_type, value
        );*/
        let column_data = vec![GenericDatasetDBMS::SQLSERVER(value)];
        data_columns.insert(col_name.to_string(), column_data);
    }
    Ok(data_columns)
}

fn query_builder(columns: &Vec<ColumnMembers>) -> String {
    columns
        .iter()
        .map(|col| {
            let col_name = col.get_column_name();
            if col.get_data_type().eq_ignore_ascii_case("hierarchyid") {
                format!("CAST([{}] as VARCHAR) as [{}]", col_name, col_name)
            } else if col.get_data_type().eq_ignore_ascii_case("xml")
                || col.get_data_type().eq_ignore_ascii_case("geography")
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
    let mut cannon_col: Vec<CanonnicalColumns> = Vec::new();
    for metadata in tables_metadata {
        let empty_otherwise = &SQLConstraints::PRIMARYKEY(IdentitySpecification::empty_struct());
        let table_key: &(String, String) = metadata.0;
        let table_metadata: &TableMetadata = metadata.1;
        println!(
            "total Rows in {}  is {}",
            table_metadata.get_table_name(),
            table_metadata.get_total_rows_as_ref()
        );
        let table_rows = *table_metadata.get_total_rows_as_ref();
        if table_rows < 10 {
            println!("on a row : {}", table_rows);
        }
        let mut next: i32 = 10;
        let mut prev = 0;
        while next <= table_rows {
            next += 10;
            if next > table_rows {
                let res = next - table_rows;
                next = next - res;
            }
            println!("Current {} | Prev {}", next, prev);
            //Do the query!
            let pk_identifier = table_metadata
                .get_constrs_as_ref()
                .iter()
                .find(|pred| match pred {
                    PRIMARYKEY(_) => true,
                    _ => false,
                })
                .unwrap_or(empty_otherwise);
            let columns_query = query_builder(table_metadata.get_cols_as_ref());
            let query_build = format!(
                "SELECT {} FROM [{}].[{}] ORDER BY [{}]  OFFSET {} ROWS FETCH NEXT {} ROWS ONLY;",
                columns_query,
                table_key.1,
                table_key.0,
                prev, //Offset
                next, // Next
                pk_identifier
                    .get_pk_ref_opt()
                    .unwrap()
                    .get_col_name_as_ref()
            );
            let rows_tables = connection
                .query(query_build, &[])
                .await
                .unwrap()
                .into_first_result()
                .await
                .unwrap();
            for row in rows_tables.iter() {
                let canonical_row = rows_to_canonnical(&row).unwrap();
                cannon_col.push(CanonnicalColumns::new(
                    table_key.0.to_string(),
                    canonical_row,
                ));
            }
            prev = next;
            if next == table_rows {
                break;
            }
        }
    }
    Ok(true)
}
