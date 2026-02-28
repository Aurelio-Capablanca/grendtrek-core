use crate::internals::data_structures::{
    database_connector_spec::DatabaseHandlers, database_metadata::db_metadata::ColumnMembers,
    database_types::query::Query,
};

pub async fn get_all_schemas(
    connection: &mut DatabaseHandlers,
    query: &Query,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut schema_names: Vec<String> = Vec::new();
    match connection {
        DatabaseHandlers::SqlServer(client) => {
            let client_mssql = &mut client.client;
            let get_schemas: Vec<tiberius::Row> = client_mssql
                .query(query.query_out(), &[])
                .await
                .unwrap()
                .into_first_result()
                .await
                .unwrap();
            get_schemas.iter().for_each(|data| {
                let schema_name: &str = data.get("SCHEMA_NAME").unwrap_or("");
                schema_names.push(schema_name.to_string());
            });
        }
        _ => {
            println!("Pass a SQL Server Connection Type at GrendTrek type management!")
        }
    }
    Ok(schema_names)
}

pub async fn get_table_by_schema(
    connection: &mut DatabaseHandlers,
    schemas: &Vec<String>,
    query_usage: &Query,
) -> Result<Vec<ColumnMembers>, Box<dyn std::error::Error>> {
    let forced_db_name = "AdventureWorks2022";
    let mut data_schema: Vec<ColumnMembers> = Vec::new();
    match connection {
        DatabaseHandlers::SqlServer(client) => {
            let client_mssql = &mut client.client;
            for name in schemas {
                let query = client_mssql
                    .query(query_usage.query_out(), &[&forced_db_name, name])
                    .await
                    .unwrap()
                    .into_first_result()
                    .await
                    .unwrap();
                for row in query {
                    let column_name: &str = row.get("column_name").unwrap_or_else(|| "");
                    let data_type: &str = row.get("data_type").unwrap_or_else(|| "");
                    let length_field: i32 = row.get("length_field").unwrap_or_else(|| 0);                    
                    let is_nullable: bool = if row
                        .get("is_nullable")
                        .unwrap_or_else(|| "")
                        .eq_ignore_ascii_case("NO")
                    {
                        false
                    } else {
                        true
                    };                   
                    let numeric_precision: i32 = row.get("numeric_precision").unwrap_or_else(|| 0);
                    let numeric_scale: i32 = row.get("numeric_scale").unwrap_or_else(|| 0);
                    // data_schema.push(ColumnMembers::new(
                    //     Some(column_name.to_string()),
                    //     Some(data_type.to_string()),
                    //     Some(length_field),
                    //     Some(is_nullable),                        
                    //     Some(numeric_precision),
                    //     Some(numeric_scale),
                    // ));
                }
            }
        }
        _ => {}
    }
    Ok(data_schema)
}
