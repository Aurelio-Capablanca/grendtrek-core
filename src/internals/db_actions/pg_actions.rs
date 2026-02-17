use crate::internals::data_structures::database_connector_spec::DatabaseHandlers;

pub async fn create_schemas(
    connection: &mut DatabaseHandlers,
    schema_names: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match connection {
        DatabaseHandlers::Postgres(client) => {
            let pg_client = &mut client.client;
            let tx = pg_client.transaction().await.unwrap();
            for schema_name in schema_names {
                let raw = format!("CREATE SCHEMA {}",schema_name);
                let query = tx.execute(&raw, &[]).await;
                match query {
                    Ok(_) => {
                        println!("Schema created ! {:?}",schema_name)
                    }
                    Err(err) =>{
                        tx.rollback().await?;
                        return Err(Box::new(err));
                    }                    
                }   
            }            
            tx.commit().await.unwrap();
        }
        _=> {}
    }
    Ok(())
}



pub async fn create_tables(){}