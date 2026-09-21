use std::collections::HashMap;

use bb8_tiberius::ConnectionManager;
use futures_util::TryStreamExt;
use tiberius::{
    ColumnType::{self},
    QueryItem, Row, Uuid,
    numeric::Numeric,
    time::chrono::{NaiveDate, NaiveDateTime},
};

use crate::internals::{
    data_structures::database_metadata::{
        constraint_metadata::{
            IdentitySpecification,
            SQLConstraints::{self, PRIMARYKEY},
        },
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
        table_data::{CanonnicalColumns, GenericDataSQLServer, GenericDatasetDBMS},
    },
    utilities::file_writer::write_to_file_os,
};

fn rows_to_canonnical(row: &Row) -> Result<HashMap<String, GenericDatasetDBMS>, Box<String>> {
    let mut data_columns: HashMap<String, GenericDatasetDBMS> = HashMap::new();
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
            _ => return Err(Box::new(String::new())),
        };
        data_columns.insert(col_name.to_string(), GenericDatasetDBMS::SQLSERVER(value));
    }
    Ok(data_columns)
}

fn column_query_builder(columns: &Vec<ColumnMembers>) -> String {
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

fn query_build_insertions(columns: &CanonnicalColumns) -> String {
    let mut batch = String::new();
    batch.push_str("INSERT INTO ");
    batch.push_str(columns.get_table_ref());
    batch.push_str(" (");
    let cols: String = columns.get_keys_as_joined_cols();
    batch.push_str(&cols);
    batch.push_str(") VALUES (");
    for (key, val) in columns.get_data_ref(){
        //mark data for it's type ('' for Strings and Dates)
        let value_insert = match val {
            GenericDatasetDBMS::SQLSERVER(values) => match values {
                GenericDataSQLServer::Text(text) => format!("'{:?}'",text),
                _=> "".to_string()                
            },
            _=> "".to_string()
        };
    }
    
    batch
}

pub async fn get_rows_from_tables(
    tables_metadata: &HashMap<(String, String), TableMetadata>,
    connection: &mut bb8::PooledConnection<'_, ConnectionManager>,
    row_offset: i32,
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
        if table_rows < row_offset {
            println!("on a row : {}", table_rows);
        }
        let mut next: i32 = row_offset;
        let mut prev = 0;
        while next <= table_rows {
            next += row_offset;
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
            let columns_query = column_query_builder(table_metadata.get_cols_as_ref());
            let query_build = format!(
                "SELECT {} FROM [{}].[{}] ORDER BY [{}]  OFFSET {} ROWS FETCH NEXT {} ROWS ONLY;",
                columns_query,
                table_key.1, //schema
                table_key.0, //table
                pk_identifier
                    .get_pk_ref_opt()
                    .unwrap()
                    .get_col_name_as_ref(),
                prev, //Offset
                next, // Next
            );
            let mut content_write = String::new();
            content_write.push_str(&query_build);
            let mut streams = connection.query(query_build, &[]).await?;
            while let Some(row) = streams.try_next().await? {
                match row {
                    QueryItem::Metadata(meta) => {
                        println!(
                            "Result set {} has {} columns",
                            meta.result_index(),
                            meta.columns().len()
                        );
                    }
                    QueryItem::Row(row) => {
                        let canonical_row: HashMap<String, GenericDatasetDBMS> =
                            rows_to_canonnical(&row).unwrap();
                        cannon_col.push(CanonnicalColumns::new(
                            table_key.0.to_string(),
                            canonical_row,
                        ));
                        //File Write
                        for cols in cannon_col.iter() {
                            let table_name = cols.get_table_ref();
                            content_write.push_str("TABLE NAME : ");
                            content_write.push_str(table_name);
                            content_write.push_str("\n");
                            let keys = cols.get_keys_ref();
                            keys.iter().for_each(|data| {
                                let middle = cols.get_ref_data_to_str(data.to_string());
                                println!("{}", middle);
                                content_write.push_str(data);
                                content_write.push_str(" : ");
                                content_write.push_str(&middle);
                                content_write.push_str("\n");
                            });
                        }
                        let file_name = format!(
                            "/data/Main/personal_projects/own/grendtrekk_writes_ddl/{}.txt",
                            table_key.0
                        );
                        write_to_file_os(content_write, &file_name.to_string());
                        content_write = "".to_string();
                    } //PG_DB insertion
                }
            }
            prev = next;
            cannon_col.clear();
            if next == table_rows {
                break;
            }
        }
    }
    Ok(true)
}
