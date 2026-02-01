use std::env;

mod internals;
mod outer;

use crate::internals::data_structures::data_struct::DatabaseConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let feature = env::var("FEATURES").expect("No loaded line");
    println!(" line loaded : {:?}", feature);
    //PostgreSQL Tester
    let pg_connection = DatabaseConnector{
        database_name: "transcontinentalshippings".to_string(),
        database_user: "superuserp".to_string(),
        database_pass: "jkl555".to_string(),
        database_host: "localhost".to_string(),
        database_port: "5432".to_string(),
    };
    let pg_guest = internals::connections::connector::create_posgres_conn();
    let client = pg_guest.await.unwrap();
    let tester = client.client.query("SELECT 1", &[])
        .await
        .unwrap();
    for test in tester {
        let resp : i32 = test.get(0);
        println!("resp {:?}",resp)
    }
    //SQL Server Tester
    
    Ok(())
}
