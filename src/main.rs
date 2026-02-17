use std::{env, io::Write};

mod internals;
mod outer;

use crate::internals::{
    connections::connector::generate_connections,
    data_structures::{
        database_connector_spec::{DatabaseConnector, DatabaseHandlers, VendorOptions},
        db_reg::DatabaseRegistry,
        database_types::query::Query,
        database_types::types::TypeMapper,
    },
    db_actions, translator,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //.ENV settings:
    dotenvy::dotenv().ok();
    let feature = env::var("FEATURES").expect("No loaded line");
    let type_path_file = env::var("TYPES_PATH").expect("translation-types.json");
    let query_path_file = env::var("QUERY_PATH").expect("translation-types.json");
    println!(
        "line loaded : {:?} , {:?}, {:?}",
        feature, type_path_file, query_path_file
    );

    //Read SQL Load Queries
    let json_data: String = std::fs::read_to_string(query_path_file).unwrap_or("".to_string());
    let queries: Vec<Query> = serde_json::from_str(&json_data).unwrap_or(Vec::new());
    queries.iter().for_each(|data| {
        println!("{:?}", data);
    });
    //
    // Load Conversion Types
    let json_types: String = std::fs::read_to_string(type_path_file).unwrap_or("".to_string());
    let type_usages: Vec<TypeMapper> = serde_json::from_str(&json_types).unwrap_or(Vec::new());
    type_usages.iter().for_each(|data| println!("{:?}", data));
    //Non Elemental Access; Test actions
    //PostgreSQL Tester
    let pg_connection = DatabaseConnector {
        database_name: "transcontinentalshippings".to_string(),
        database_user: "superuserp".to_string(),
        database_pass: "jkl555".to_string(),
        database_host: "localhost".to_string(),
        database_port: "5432".to_string(),
    };
    let handler_destiny: DatabaseHandlers =
        generate_connections(pg_connection, VendorOptions::POSTGRES)
            .await
            .unwrap_or(DatabaseHandlers::None);
    let mssql_connection = DatabaseConnector {
        database_name: "AdventureWorks2022".to_string(),
        database_user: "sa".to_string(),
        database_pass: "jklgHnbvc555SS".to_string(),
        database_host: "localhost".to_string(),
        database_port: "1433".to_string(),
    };
    let handler_origin: DatabaseHandlers =
        generate_connections(mssql_connection, VendorOptions::MSSQL)
            .await
            .unwrap_or(DatabaseHandlers::None);

    let mut entry_registries = DatabaseRegistry {
        origin: handler_origin,
        destiny: handler_destiny,
    };
    //separate connections
    let origin = &mut entry_registries.origin;
    let destiny = &mut entry_registries.destiny;

    //Get Schemas Action
    let schema_query: &Query = queries.iter().find(|&pred| match pred.engine_out() {
        VendorOptions::MSSQL => true,
        _ => false,
    } && pred.id_out().eq(&2)
    ).unwrap();
    //println!("{:?}", schema_query);
    let schema_names = db_actions::sql_server_actions::get_all_schemas(origin, schema_query)
        .await
        .unwrap_or(Vec::new());
    //schema_names.iter().for_each(|data| println!("{:?}", data));

    //try create Schemas
    let create_schemas = db_actions::pg_actions::create_schemas(destiny, &schema_names).await;
    match create_schemas {
        Ok(_) => {
            println!("Success!")
        }
        Err(err) => {
            println!("Error as : {:?}", err)
        }
    }

    //Get Tables Data
    let tables_query : &Query = queries.iter().filter(|pred| match pred.engine_out() {
        VendorOptions::MSSQL => true,
        _=> false
    } && pred.id_out().eq(&3) ).next().unwrap();
    //println!("{:?}", tables_query);
    let data_tables =
        db_actions::sql_server_actions::get_table_by_schema(origin, &schema_names, tables_query)
            .await
            .unwrap_or(Vec::new());

    //Generate DDL for creating Tables
    let types_translation: Vec<&TypeMapper> = type_usages
        .iter()
        .filter(|pred| {
            matches!(pred.get_origin_engine(), VendorOptions::MSSQL)
                && matches!(pred.get_destiny_engine(), VendorOptions::POSTGRES)
        })
        .collect::<Vec<&TypeMapper>>();

    let ddl_issue: Vec<String> =
        translator::sql_server_to_pg::translate_ddl(&data_tables, types_translation)
            .unwrap_or(Vec::new());
    //ddl_issue.iter().for_each(|data| println!("DDL : {:?}",data));

    let write = ddl_issue.join(" \n");
    let file_exists =
        std::fs::metadata("/main_root/personal_projects/own/grendtrekk_writes_ddl/ddl.sql");

    let file_ddl = match file_exists {
        Ok(metadata) => {
            println!("{:?}", metadata);
            std::fs::File::create("/main_root/personal_projects/own/grendtrekk_writes_ddl/ddl.sql")
        }
        Err(_) => {
            println!("File Already Exists, only writting at it");
            std::fs::File::create_new(
                "/main_root/personal_projects/own/grendtrekk_writes_ddl/ddl.sql",
            )
        }
    };

    match file_ddl {
        Ok(mut file) => match file.write_all(write.as_bytes()) {
            Ok(_) => {
                print!("OK")
            }
            Err(err) => {
                println!("Error at placing data : {:?}", err)
            }
        },
        Err(err) => {
            println!("Error Writting file log DDL : {:?}", err)
        }
    }
    
    Ok(())
}
