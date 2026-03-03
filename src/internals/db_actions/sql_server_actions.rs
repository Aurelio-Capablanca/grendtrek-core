use std::collections::HashMap;

use tiberius::Row;

use crate::internals::data_structures::{
    database_connector_spec::{DatabaseHandlers, MSSQLHandler},
    database_metadata::{
        constraint_metadata::{ComputedSpecification, SQLConstraints},
        db_metadata::{cannonical_columns::ColumnMembers, cannonical_tables::TableMetadata},
    },
    database_types::query::Query,
};

// pub async fn get_all_schemas(
//     connection: &mut DatabaseHandlers,
//     query: &Query,
// ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let mut schema_names: Vec<String> = Vec::new();
//     match connection {
//         DatabaseHandlers::SqlServer(client) => {
//             let client_mssql = &mut client.client;
//             let get_schemas: Vec<tiberius::Row> = client_mssql
//                 .query(query.query_out(), &[])
//                 .await
//                 .unwrap()
//                 .into_first_result()
//                 .await
//                 .unwrap();
//             get_schemas.iter().for_each(|data| {
//                 let schema_name: &str = data.get("SCHEMA_NAME").unwrap_or("");
//                 schema_names.push(schema_name.to_string());
//             });
//         }
//         _ => {
//             println!("Pass a SQL Server Connection Type at GrendTrek type management!")
//         }
//     }
//     Ok(schema_names)
// }

// pub async fn get_table_by_schema(
//     connection: &mut DatabaseHandlers,
//     schemas: &Vec<String>,
//     query_usage: &Query,
// ) -> Result<Vec<ColumnMembers>, Box<dyn std::error::Error>> {
//     let forced_db_name = "AdventureWorks2022";
//     let mut data_schema: Vec<ColumnMembers> = Vec::new();
//     match connection {
//         DatabaseHandlers::SqlServer(client) => {
//             let client_mssql = &mut client.client;
//             for name in schemas {
//                 let query = client_mssql
//                     .query(query_usage.query_out(), &[&forced_db_name, name])
//                     .await
//                     .unwrap()
//                     .into_first_result()
//                     .await
//                     .unwrap();
//                 for row in query {
//                     let column_name: &str = row.get("column_name").unwrap_or_else(|| "");
//                     let data_type: &str = row.get("data_type").unwrap_or_else(|| "");
//                     let length_field: i32 = row.get("length_field").unwrap_or_else(|| 0);
//                     let is_nullable: bool = if row
//                         .get("is_nullable")
//                         .unwrap_or_else(|| "")
//                         .eq_ignore_ascii_case("NO")
//                     {
//                         false
//                     } else {
//                         true
//                     };
//                     let numeric_precision: i32 = row.get("numeric_precision").unwrap_or_else(|| 0);
//                     let numeric_scale: i32 = row.get("numeric_scale").unwrap_or_else(|| 0);
//                     // data_schema.push(ColumnMembers::new(
//                     //     Some(column_name.to_string()),
//                     //     Some(data_type.to_string()),
//                     //     Some(length_field),
//                     //     Some(is_nullable),
//                     //     Some(numeric_precision),
//                     //     Some(numeric_scale),
//                     // ));
//                 }
//             }
//         }
//         _ => {}
//     }
//     Ok(data_schema)
// }

struct MSSQLCols<'a> {
    col: Option<&'a Row>,
}

pub async fn build_canonnical_schema(
    connection: &mut DatabaseHandlers,
    query_list: Vec<&Query>,
) -> Result<Vec<TableMetadata>, Box<dyn std::error::Error>> {
    match connection {
        DatabaseHandlers::SqlServer(client) => {
            let client_conn = &mut client.client;
            //Get Schemas
            let col_query = query_list.iter().find(|pred| pred.id_out().eq(&2)).unwrap();
            let cols = client_conn
                .query(col_query.query_out(), &[])
                .await
                .unwrap()
                .into_first_result()
                .await
                .unwrap();
            let mut tables: HashMap<(String, String), TableMetadata> = HashMap::new();
            for col in cols {
                let table_name = col
                    .get("table_name")
                    .map(str::to_owned)
                    .unwrap_or_else(|| "no_table".to_owned());
                let schema_name = col
                    .get("schmea_name")
                    .map(str::to_owned)
                    .unwrap_or_else(|| "no_schema".to_owned());
                //--
                let ordering: Option<i32> = col.get("ordering");
                let col_name: Option<&str> = col.get("column_name");
                let data_type: Option<&str> = col.get("data_type");
                let length_field: Option<i32> = col.get("length_field");
                let numeric_precision: Option<i32> = col.get("type_precision");
                let numeric_scale: Option<i32> = col.get("decimal_scale");
                let collation: Option<&str> = col.get("is_nullable");
                let is_nullable: Option<bool> = col.get("is_nullable");
                let is_identity: Option<bool> = col.get("identity_col");
                let is_gen_always: Option<bool> = col.get("generated_always");
                let text_gen_alw: Option<&str> = col.get("text_generated_always");
                let def_comp_val: Option<&str> = col.get("computed_col_value");
                let key_table = (table_name.clone(), schema_name.clone());
                let column_memb = ColumnMembers::new(
                    Some(col_name.map(str::to_owned).unwrap()),
                    Some(data_type.map(str::to_owned).unwrap()),
                    length_field,
                    numeric_precision,
                    numeric_scale,
                    Some(collation.map(str::to_owned).unwrap()),
                    is_nullable,
                    is_identity,
                    is_gen_always,
                    Some(text_gen_alw.map(str::to_owned).unwrap()),
                );

                let computed_expression: Vec<SQLConstraints> = if let Some(val) = def_comp_val {
                    vec![SQLConstraints::COMPUTED(ComputedSpecification::new(
                        Some(String::new()),
                        val.to_string(),
                        Some(false),
                        Some(false),
                        Some(false),
                        col.get("column_name")
                            .map(str::to_owned)
                            .unwrap_or_else(|| "".to_string()),
                    ))]
                } else {
                    Vec::new()
                };

                tables
                    .entry(key_table)
                    .and_modify(|lambda| {
                        lambda.add_columns(column_memb.clone());
                    })
                    .or_insert_with_key(|_| {
                        TableMetadata::new(
                            table_name,
                            schema_name,
                            vec![column_memb],
                            computed_expression,
                            Vec::new(),
                        )
                    });
            }
            //Get Cannonical Columns
            //Get Cannonical PK
            //Get Cannonical FK
            //Get Cannonical Indexes
            //Get Canonnical Constraints (Default/Check)
        }
        _ => {}
    }
    Ok(Vec::new())
}
