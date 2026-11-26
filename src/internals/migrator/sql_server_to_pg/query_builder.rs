use std::collections::HashMap;

use std::error::Error;
use std::sync::LazyLock;

use bb8::{Pool, PooledConnection};
use bb8_tiberius::ConnectionManager;
use futures_util::TryStreamExt;
use futures_util::future::join_all;
use tiberius::QueryItem;
use tiberius::time::chrono::{DateTime, FixedOffset, NaiveTime};
use tiberius::{
    ColumnType::{self},
    Uuid,
    numeric::Numeric,
    time::chrono::{NaiveDate, NaiveDateTime},
};
use tokio::task::JoinHandle;

use crate::internals::data_structures::database_metadata::{
    constraint_metadata::IdentitySpecification,
    db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
};
use crate::utilities::file_writer;

static EMPTY_OTHERWISE: LazyLock<IdentitySpecification> =
    LazyLock::new(|| IdentitySpecification::empty_struct());

/**
 Work at gathering all the columns without too much recomputation from the original execution at
 "column_query_builder" -> line 33
*/
fn build_insertion_dml_sentence(
    stream: QueryItem, //call here type conversion!
) -> Result<String, Box<dyn std::error::Error>> {
    let mut dml_batch = String::new();
    match stream {
        QueryItem::Row(row) => {
            for (i, col) in row.columns().iter().enumerate() {
                let cols_size = row.len();
                if i.eq(&0) {
                    dml_batch.push_str(" (");
                }
                let col_type = col.column_type();
                let value: String = match col_type {
                    ColumnType::Null => "NULL".to_string(),
                    ColumnType::Bit => {
                        let value: Option<bool> = row.get(i);
                        match value {
                            Some(val) => format!("{}", val),
                            None => "false".to_string(),
                        }
                    }
                    ColumnType::Int1 => {
                        //
                        let value: Option<u8> = row.get(i);
                        match value {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Int2 => {
                        let value: Option<i16> = row.get(i);
                        match value {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Int4 => {
                        let value: Option<i32> = row.get(i);
                        match value {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Int8 | ColumnType::Intn => {
                        let val_int8: Option<i64> = row.get(i);
                        match val_int8 {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Datetime4 => {
                        let val_date4: Option<NaiveDateTime> = row.get(i);
                        match val_date4 {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Float4 => {
                        let val_float4: Option<f32> = row.get(i);
                        match val_float4 {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Float8 => {
                        let val_float8: Option<f64> = row.get(i);
                        match val_float8 {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Money => {
                        let val_money: Option<f64> = row.get(i);
                        match val_money {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Money4 => {
                        let val_money4: Option<f32> = row.get(i);
                        match val_money4 {
                            Some(val) => format!("{}", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Datetime => {
                        let val_datetime: Option<NaiveDateTime> = row.get(i);
                        match val_datetime {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Guid => {
                        let guid: Option<Uuid> = row.get(i);
                        match guid {
                            Some(val) => format!("'{}'", val),
                            None => "NUll".to_string(),
                        }
                    }
                    ColumnType::Bitn => {
                        let bit_n: Option<i64> = row.get(i);
                        match bit_n {
                            Some(val) => format!("{}", val),
                            None => "NUll".to_string(),
                        }
                    }
                    ColumnType::Numericn | ColumnType::Decimaln => {
                        let numeric_n: Numeric =
                            row.get(i).unwrap_or_else(|| Numeric::new_with_scale(0, 0));
                        let translated = f64::from(numeric_n);
                        format!("{}", translated)
                    }
                    ColumnType::Floatn => {
                        let float_n: Option<f64> = row.get(i);
                        match float_n {
                            Some(val) => format!("{}", val),
                            None => "NUll".to_string(),
                        }
                    }
                    ColumnType::Datetimen => {
                        let val_datetime: Option<NaiveDateTime> = row.get(i);
                        match val_datetime {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Daten => {
                        let val_datetime: Option<NaiveDate> = row.get(i);
                        match val_datetime {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Timen => {
                        let val_time: Option<NaiveTime> = row.get(i);
                        match val_time {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::Datetime2 => {
                        let val_datetime: Option<NaiveDateTime> = row.get(i);
                        match val_datetime {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::DatetimeOffsetn => {
                        let val_datetime_offset: Option<DateTime<FixedOffset>> = row.get(i);
                        match val_datetime_offset {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::BigVarBin => {
                        let val: Option<Vec<u8>> = row.get::<&[u8], _>(i).map(|b| b.to_vec());
                        match val {
                            Some(v) => {
                                let string_concat =
                                    String::from_utf8(v).unwrap_or("NULL".to_string());
                                format!("'{}'", string_concat)
                            }
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::BigChar | ColumnType::NVarchar => {
                        let val_big_char: Option<&str> = row.get(i);
                        match val_big_char {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    ColumnType::NChar => {
                        let val_nv_char: Option<&str> = row.get(i);
                        match val_nv_char {
                            Some(val) => format!("'{}'", val),
                            None => "NULL".to_string(),
                        }
                    }
                    _ => "NULL".to_string(),
                };
                dml_batch.push_str(value.as_str());
                if cols_size - 1 == i {
                    dml_batch.push_str("),");
                } else {
                    dml_batch.push_str(",");
                }
                //reconsider strategy at calling per row call!
            }
            dml_batch.push_str("\n");
        }
        _ => {}
    }
    Ok(dml_batch)
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


pub async fn get_rows_to_issue_dml(
    tables_metadata: HashMap<(String, String), TableMetadata>,
    connection: Pool<ConnectionManager>,
    row_offset: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut handlers : Vec<JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>> = Vec::new();

    for (keys, metadata) in tables_metadata {
        let pool = connection.clone();       
        
        let handler = tokio::spawn(async move {            
            let mut conn : PooledConnection<'_, ConnectionManager> = pool.get().await?;
            
            let table_key: (String, String) = keys;
            let table_metadata: TableMetadata = metadata;
            let table_rows = table_metadata.get_total_rows_as_ref().clone();
            let mut next: i32 = row_offset;
            let mut prev = 0;

            let pk_identifier = table_metadata
                .get_pk_as_ref()
                .iter()
                .find(|pred| pred.get_table_name_as_ref().eq(&table_key.0))
                .unwrap_or(&EMPTY_OTHERWISE)
                .get_col_name_as_ref();

            while next <= table_rows {
                // consider insert here logic for spawning threads
                if table_rows >= 1000 {
                    next += row_offset;
                    if next > table_rows {
                        let res = next - table_rows;
                        next = next - res;
                    }
                } else {
                    next = table_rows;
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
                let mut content_write = String::new();
                content_write.push_str(&query_build);
                //Execute Query!
                let mut streams = conn.query(query_build, &[]).await.unwrap();
                content_write.push_str(
                    format!(
                        "\n INSERT INTO {}.{} (",
                        table_key.0.as_str(),
                        table_key.1.as_str()
                    )
                    .as_str(),
                );
                // logic needed to iterate on cols
                content_write.push_str(" ) \n VALUES");
                while let Some(stream) = streams.try_next().await.unwrap() {
                    let dml = build_insertion_dml_sentence(stream);
                    let string_dml: String = match dml {
                        Ok(data) => data,
                        Err(err) => {
                            panic!("Error at getting metadata! {}", err)
                        }
                    };
                    content_write.push_str(string_dml.as_str());
                }
                //let batch = query_build_insertions(&cannon_col, &table_key.0, &table_key.1);
                let file_name = format!(
                    "/data/Main/personal_projects/own/grendtrekk_writes_ddl/{}-offset{}-next{}.txt",
                    table_key.0,
                    prev, //Offset
                    next, // Next
                );
                file_writer::write_to_file_os(content_write, &file_name.to_string());
                //clear actions
                content_write = "".to_string();
                prev = next;
                if next == table_rows {
                    break;
                }
            }
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });
        
        handlers.push(handler);
    }
    
    let gatherers = join_all(handlers).await;
    
    for gatherer in gatherers {
        match gatherer {
            Ok(result) => result.unwrap(),
            Err(err) => return Err(Box::new(err))
        }
    }
    
    Ok(true)
}
