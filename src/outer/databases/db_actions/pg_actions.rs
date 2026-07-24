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
            for schema in schema_names {
                let query = tx.execute(schema, &[]).await;
                match query {
                    Ok(_) => {
                        println!("Schema created ! {:?}", schema)
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
            Ok(coll) => {
                println!("Collation Created ! {} {}", colls, coll)
            }
            Err(err) => {
                tx.rollback().await?;
                println!("Collation not created ! {}", colls);
                eprintln!("Error as : {:?}", err);
                return Err(Box::new(err));
            }
        }
    }
    tx.commit().await.unwrap();
    Ok(true)
}

pub async fn create_tables(
    ddl_instruction: &Vec<String>,
    connection: &mut PgPoolHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn_pg = connection.pg_pool.get().await.unwrap();
    let tx = conn_pg.transaction().await.unwrap();
    for ddl in ddl_instruction {
        let ddl_query = tx.execute(ddl, &[]).await;
        match ddl_query {
            Ok(res) => {
                println!("Table created {} | DB signal {}",ddl,res)
            }
            Err(err) => {                
                eprintln!("Error at Creating table : {:?}", err);
                tx.rollback().await.unwrap();
                return Err(Box::new(err));
            }
        }
    }
    tx.commit().await.unwrap();
    Ok(())
}
