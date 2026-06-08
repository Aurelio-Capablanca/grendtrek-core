use std::{
    collections::{HashMap, HashSet},
    env,
    io::Write,
};

mod internals;
mod outer;

use crate::{
    internals::{
        data_structures::{
            database_connector_spec::{DatabaseConnector, DatabaseHandlers, VendorOptions},
            database_metadata::{
                db_metadata::cannonical_tables::TableMetadata, /*table_data::TableData,*/
            },
            database_types::{query::Query, types::TypeMapper},
            db_reg::DatabaseRegistry,
        },
        translator::sql_server_to_pg,
    },
    outer::databases::db_actions::pg_actions,
};

use crate::outer::databases::{
    connections::connector::generate_connections, db_actions::sql_server_actions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Don't panic!
    // let table_value: TableData<i32> = TableData::new("a_table".to_string(), "a_value".to_string(), 100_i32);
    // print!("{:?}",table_value);

    //.ENV settings:
    dotenvy::dotenv().ok();
    let feature = env::var("FEATURES").expect("No loaded line");
    let type_path_file = env::var("TYPES_PATH").expect("translation-types.json");
    let query_path_file = env::var("QUERY_PATH").expect("queries-config.json");
    println!(
        "line loaded : {:?} , {:?}, {:?}",
        feature, type_path_file, query_path_file
    );

    //Read SQL Load Queries
    let json_data: String = std::fs::read_to_string(query_path_file).unwrap_or("".to_string());
    let queries: Vec<Query> = serde_json::from_str(&json_data).unwrap_or(Vec::new());
    queries.iter().for_each(|data| {
        println!("Queries : {:?}", data);
    });
    //
    // Load Conversion Types
    let json_types: String = std::fs::read_to_string(type_path_file).unwrap_or("".to_string());
    let type_usages: Vec<TypeMapper> = serde_json::from_str(&json_types).unwrap_or(Vec::new());
    type_usages
        .iter()
        .for_each(|data| println!("Types: {:?}", data));

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

    let sqlserver_cannon: Vec<&Query> = queries
        .iter()
        .filter(|pred| match pred.engine_out() {
            VendorOptions::MSSQL => true,
            _ => false,
        })
        .collect::<Vec<&Query>>();
    let mut canonnical_model: HashMap<(String, String), TableMetadata> =
        sql_server_actions::build_canonnical_schema(origin, sqlserver_cannon).await?;

    canonnical_model.iter().for_each(|data| {
        let key = data.0;
        let value = data.1;
        println!("{:?} \n {:?}", key, value);
    });

    // create schemas
    let schemas_cannonical: HashSet<String> = canonnical_model
        .iter()
        .map(|data| {
            let schema = data.0;
            schema.1.clone()
        })
        .collect::<HashSet<_>>();
    schemas_cannonical
        .iter()
        .for_each(|data| println!("{:?}", data));
    // 1.1 send to postgres
    let action = pg_actions::create_schemas(destiny, &schemas_cannonical).await;
    match action {
        Ok(_) => {
            println!("Schemas Created!");
        }
        Err(err) => {
            println!("Error at creating schemas : {:?}", err)
        }
    }
    // issue ddl
    let type_conversion = type_usages.iter().filter(|pred| match pred.get_origin_engine()  {
            VendorOptions::MSSQL => true,
            _=> false
        }  &&
        match  pred.get_destiny_engine() {
            VendorOptions::POSTGRES => true,
            _=> false,
        }
        ).collect::<Vec<&TypeMapper>>();
    let ddl_for_pg = match sql_server_to_pg::translate_ddl(&mut canonnical_model, type_conversion) {
        Ok(value) => {
            value.iter().for_each(|data| println!("{:?} \n", data));
            value
        }
        Err(err) => {
            println!("{:?}", err);
            Vec::new()
        }
    };

    // mock save the results of the DDL reconstruction
    let write = ddl_for_pg.join(" \n");
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

    // create tables, fk's and pk's

    // create indexes (alter table)
    // create default values
    // get bulks
    // insert bulks
    // create check values
    // finish trekk

    // test for actions

    Ok(())
}
