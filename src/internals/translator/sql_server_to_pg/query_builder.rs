use std::{collections::HashMap, io::Error};

use bb8_tiberius::ConnectionManager;
use futures_util::TryStreamExt;
use tiberius::{
    ColumnType::{self},
    Row, Uuid,
    numeric::Numeric,
    time::chrono::{NaiveDate, NaiveDateTime},
};
use tiberius::{QueryItem, QueryStream};
use tokio_util::io::simplex::new;

use crate::internals::{
    data_structures::database_metadata::{
        constraint_metadata::IdentitySpecification,
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
        table_data::{CanonnicalColumns, GenericDataSQLServer, GenericDatasetDBMS},
    },
    utilities::file_writer::write_to_file_os,
};

async fn query_stream_to_canonnical(
    mut stream: QueryItem,
) -> Result<Vec<(String, GenericDatasetDBMS)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(String, GenericDatasetDBMS)> = Vec::new();
    let metadata = stream.as_metadata().unwrap();
    let row = stream.as_row().unwrap();
    for col in metadata.columns() {
        let i = col.name();
        let value = match col.column_type() {
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
            _ => {
                return Err(Box::new(Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "Error parsing data from Origin",
                )));
            }
        };
    }
    Ok(result)
}

fn query_build_insertions(
    columns: &Vec<CanonnicalColumns>,
    table_name: &str,
    schema_table: &str,
) -> String {
    let mut batch = String::new();
    batch.push_str("INSERT INTO ");
    batch.push_str(format!("{}.{}", table_name, schema_table).as_str());
    batch.push_str(" (");
    let first_col = columns.first().unwrap();
    let cols: String = first_col.get_keys_as_joined_cols();
    batch.push_str(&cols);
    batch.push_str(") VALUES ");
    columns.iter().for_each(|cols_data| {
        let fields: &Vec<(_, GenericDatasetDBMS)> = cols_data.get_data_ref();
        let list_size: usize = fields.len();
        for (i, (_, val)) in fields.iter().enumerate() {
            batch.push_str(" (");
            //mark data for it's type ('' for Strings and Dates)
            let value_insert = match val {
                GenericDatasetDBMS::SQLSERVER(values) => match values {
                    GenericDataSQLServer::Text(text) => {
                        //format!("'{}'", text.as_ref().unwrap_or(&String::new()))
                        if text.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("'{}'", text.as_ref().unwrap())
                        }
                    }
                    //times
                    GenericDataSQLServer::Date(date) => {
                        if date.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("'{}'", date.as_ref().unwrap())
                        }
                    }
                    GenericDataSQLServer::DateTimeLocal(datelocal) => {
                        if datelocal.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("'{}'", datelocal.as_ref().unwrap())
                        }
                    }
                    //binaries
                    GenericDataSQLServer::BigBinary(binary) => {
                        if binary.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("'{:?}'", binary.as_ref().unwrap())
                        }
                    }
                    GenericDataSQLServer::Bit(bits) => {
                        if bits.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("'{}'", bits.as_ref().unwrap())
                        }
                    }
                    //numerics
                    GenericDataSQLServer::Int(ints) => {
                        if ints.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("{}", ints.as_ref().unwrap())
                        }
                    }
                    GenericDataSQLServer::SmallInt(sint) => {
                        if sint.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("{}", sint.as_ref().unwrap())
                        }
                    }
                    GenericDataSQLServer::Float(floats) => {
                        if floats.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("{}", floats.as_ref().unwrap())
                        }
                    }
                    GenericDataSQLServer::Bool(boolean) => {
                        if boolean.as_ref().is_none() {
                            "NULL".to_string()
                        } else {
                            format!("{}", boolean.as_ref().unwrap())
                        }
                    }
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            };
            batch.push_str(&value_insert);
            if list_size - 1 == i {
                batch.push_str(");");
            } else {
                batch.push_str("),");
            }
        }
    });
    batch
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

pub async fn get_rows_from_tables(
    tables_metadata: &HashMap<(String, String), TableMetadata>,
    connection: &mut bb8::PooledConnection<'_, ConnectionManager>,
    row_offset: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cannon_col: Vec<CanonnicalColumns> = Vec::new();
    for metadata in tables_metadata {
        let table_key: &(String, String) = metadata.0;
        let table_metadata: &TableMetadata = metadata.1;
        let table_rows = *table_metadata.get_total_rows_as_ref();
        let mut next: i32 = row_offset;
        let mut prev = 0;
        let empty_otherwise = IdentitySpecification::empty_struct();
        let pk_identifier = table_metadata
            .get_pk_as_ref()
            .iter()
            .find(|pred| pred.get_table_name_as_ref().eq(&table_key.0))
            .unwrap_or(&empty_otherwise)
            .get_col_name_as_ref();
        while next <= table_rows {
            next += row_offset;
            if next > table_rows {
                let res = next - table_rows;
                next = next - res;
            }
            //Do the query!
            let columns_query = column_query_builder(table_metadata.get_cols_as_ref());
            let query_build = format!(
                "SELECT {} FROM [{}].[{}] ORDER BY [{}]  OFFSET {} ROWS FETCH NEXT {} ROWS ONLY;",
                columns_query,
                table_key.1, //schema
                table_key.0, //table
                pk_identifier,
                prev, //Offset
                next, // Next
            );
            //println!("Query exec : {}", query_build);
            let mut content_write = String::new();
            content_write.push_str(&query_build);
            //Execute Query!
            let mut streams = connection.query(query_build, &[]).await?;
            while let Some(stream) = streams.try_next().await? {
                let middle = query_stream_to_canonnical(stream);
            }
            //let batch = query_build_insertions(&cannon_col, &table_key.0, &table_key.1);
            //content_write.push_str(&format!("\n{}", &batch));
            let file_name = format!(
                "/data/Main/personal_projects/own/grendtrekk_writes_ddl/{}-offset{}-next{}.txt",
                table_key.0,
                prev, //Offset
                next, // Next
            );
            //
            // println!("schema : {} | table : {}", table_key.0, table_key.1);
            //write_to_file_os(content_write, &file_name.to_string());
            //clear actions
            content_write = "".to_string();
            prev = next;
            cannon_col.clear();
            if next == table_rows {
                break;
            }
        }
    }
    Ok(true)
}
