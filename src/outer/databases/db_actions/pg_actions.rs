use std::collections::HashSet;

use crate::internals::data_structures::database_connector_spec::{DatabaseHandlers, PgPoolHandler};

pub async fn create_schemas(
    connection: &mut DatabaseHandlers,
    schema_names: &HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match connection {
        DatabaseHandlers::PostgresPool(client) => {
            let pg_client = &mut client.pg_pool.get().await.unwrap();
            let tx = pg_client.transaction().await.unwrap();
            for schema_name in schema_names {
                let raw = format!("CREATE SCHEMA {}", schema_name);
                let query = tx.execute(&raw, &[]).await;
                match query {
                    Ok(_) => {
                        println!("Schema created ! {:?}", schema_name)
                    }
                    Err(err) => {
                        tx.rollback().await?;
                        return Err(Box::new(err));
                    }
                }
            }
            tx.commit().await.unwrap();
        }
        _ => {}
    }
    Ok(())
}

pub async fn create_new_collations(
    translated: Vec<String>,
    connection: &mut PgPoolHandler,
) -> Result<bool, Box<dyn std::error::Error>> {
    let pg_client = &mut connection.pg_pool.get().await.unwrap();
    let tx = pg_client.transaction().await.unwrap();
    for colls in translated {
        let collation_query = tx.execute(&colls, &[]).await;
        match collation_query {
            Ok(_) => {
                println!("Collation Created !")
            }
            Err(err) => {                
                tx.rollback().await?;
                return  Err(Box::new(err));
            }
        }        
    }
    tx.commit().await.unwrap();
    Ok(true)
}

//pub async fn create_tables(){}
