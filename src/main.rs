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
            database_metadata::db_metadata::cannonical_tables::TableMetadata,
            database_types::{collation::Collations, query::Query, types::TypeMapper},
            db_reg::DatabaseRegistry,
        },
        translator::sql_server_to_pg::{ddl_translation, query_builder},
    },
    outer::databases::db_actions::pg_actions,
};

use crate::outer::databases::{
    connections::connector::generate_connections, db_actions::sql_server_actions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //.ENV settings:
    dotenvy::dotenv().ok();
    let feature = env::var("FEATURES").expect("No loaded line");
    let type_path_file = env::var("TYPES_PATH").expect("translation-types.json");
    let query_path_file = env::var("QUERY_PATH").expect("queries-config.json");
    let collations_path_file = env::var("COLLATION_PATH").expect("collation-translation.json");
    println!(
        "line loaded : {:?} , {:?}, {:?}, {:?}",
        feature, type_path_file, query_path_file, collations_path_file
    );

    //Read SQL Load Queries
    let json_data: String = std::fs::read_to_string(query_path_file).unwrap_or("".to_string());
    let queries: Vec<Query> = serde_json::from_str(&json_data).unwrap_or(Vec::new());

    // Load Conversion Types
    let json_types: String = std::fs::read_to_string(type_path_file).unwrap_or("".to_string());
    let type_usages: Vec<TypeMapper> = serde_json::from_str(&json_types).unwrap_or(Vec::new());

    // Load Collation Types (only MSSQL to PG use case as current use case)
    let json_collation_or = std::fs::read_to_string(collations_path_file); //.unwrap_or("".to_string());
    let string_cont = match json_collation_or {
        Ok(value) => {
            println!("String from file! {:?}", value);
            value
        }
        Err(err) => {
            println!("Error at gathering Collations as : {:?} ", err);
            String::new()
        }
    };

    let collation_dec = serde_json::from_str(&string_cont);
    let collations: Vec<Collations> = match collation_dec {
        Ok(value) => value,
        Err(err) => {
            println!("Error in gathering Vector from JSON : {:?} ", err);
            Vec::new()
        }
    };
    if collations.is_empty() {
        println!("Error at gathering collations!")
    }
    collations
        .iter()
        .for_each(|data| println!("Collations: {:?}", data));

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
            format!("CREATE SCHEMA {:?}", schema.1.to_owned())
        })
        .collect::<HashSet<_>>();
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
    // issue new collations
    let translate_collations: Vec<String> =
        ddl_translation::build_collation_mod(&collations).unwrap_or(Vec::new());
    let pg_pool = match destiny {
        DatabaseHandlers::PostgresPool(pg) => pg,
        _ => {
            panic!("No pool found!")
        }
    };
    // 1.2 create collations
    let collation_result =
        match pg_actions::create_new_collations(translate_collations, pg_pool).await {
            Ok(res) => res,
            Err(err) => {
                eprint!("{}",err);
                false
            }
        };
    // 2.0  issue ddl pk with tables
    //  --- type translation
    let type_conversion = type_usages.iter().filter(|pred| match pred.get_origin_engine()  {
                    VendorOptions::MSSQL => true,
                    _=> false
                }  &&
                match  pred.get_destiny_engine() {
                    VendorOptions::POSTGRES => true,
                    _=> false,
                }
                ).collect::<Vec<&TypeMapper>>();
    // --- DDL generation
    let ddl_for_pg = match ddl_translation::translate_ddl(&mut canonnical_model, type_conversion) {
        Ok(value) => {
            //value.iter().for_each(|data| println!("{:?} \n", data));
            value
        }
        Err(err) => {
            println!("{:?}", err);
            Vec::new()
        }
    };
    //test type detection
    let mut connection = match origin {
        DatabaseHandlers::SqlServerPool(conn) => conn.mssql_pool.get().await.unwrap(),
        _ => {
            panic!("No connection ?")
        }
    };
    let result_types =
        query_builder::get_rows_from_tables(&canonnical_model, &mut connection).await?;
    // fk ddl
    // create indexes (alter table) ddl
    // create default values ddl
    // get bulks (query in chunks all the db data)
    // insert bulks (batch insert it!)
    // create check values
    // finish trekk
    // mock save the results of the DDL reconstruction
    let write = ddl_for_pg.join(" \n");
    let file_exists =
        std::fs::metadata("/data/Main/personal_projects/own/grendtrekk_writes_ddl/ddl.sql");
    let file_ddl = match file_exists {
        Ok(metadata) => {
            println!("{:?}", metadata);
            std::fs::File::create("/data/Main/personal_projects/own/grendtrekk_writes_ddl/ddl.sql")
        }
        Err(_) => {
            println!("File Already Exists, only writting at it");
            std::fs::File::create_new(
                "/data/Main/personal_projects/own/grendtrekk_writes_ddl/ddl.sql",
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
    // file writting for OS
    Ok(())
}
