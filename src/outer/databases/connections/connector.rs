use crate::internals::data_structures::database_connector_spec::{
    DatabaseConnector, DatabaseHandlers, MSSQLPoolHandler, PgPoolHandler,
    VendorOptions,
};
use bb8::Pool as bb8Pool;
use bb8_tiberius::ConnectionManager;
use deadpool_postgres::{Config};
use tiberius::{AuthMethod};
//use tokio::net::TcpStream;
use tokio_postgres::NoTls; 
//use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn generate_connections(
    connector: DatabaseConnector,
    vendor: VendorOptions,
) -> Result<DatabaseHandlers, Box<dyn std::error::Error>> {
    //Create
    let mut general_handler: DatabaseHandlers = match vendor {
        VendorOptions::POSTGRES => {
            let pool_pg = create_postgres_pool_connection(connector).await.unwrap();
            DatabaseHandlers::PostgresPool(pool_pg)
        }
        VendorOptions::MSSQL => {
            let client_mssql = create_mssql_pool_connection(connector).await.unwrap();
            DatabaseHandlers::SqlServerPool(client_mssql)
        }
        VendorOptions::SQLITE => {
            println!("No options available ! yet");
            DatabaseHandlers::None
        }
        VendorOptions::MYSQL => {
            println!("No options available ! yet");
            DatabaseHandlers::None
        }
        VendorOptions::NONE => {
            println!("No Option Available!");
            DatabaseHandlers::None
        }
    };
    //Handle tester
    match &mut general_handler {
        DatabaseHandlers::PostgresPool(pool_pg) => {
            let client = pool_pg.pg_pool.get().await.unwrap();
            let actions = client.query("SELECT 1", &[]).await.unwrap();
            for action in actions {
                let row: i32 = action.get(0);
                println!("Pool PG Result : {:?}", row);
            }
        }
        DatabaseHandlers::SqlServerPool(mssql_pool) => {
            let mut client = mssql_pool.mssql_pool.get().await.unwrap();
            let actions = client
                .query("SELECT 1 as result", &[])
                .await
                .unwrap()
                .into_first_result()
                .await
                .unwrap();
            for action in actions {
                let result: i32 = action.get("result").unwrap();
                println!("SQL Server Pool BB8 result : {:?} ", result)
            }
        }
        DatabaseHandlers::None => {}
    }
    Ok(general_handler)
}

async fn create_postgres_pool_connection(
    connector: DatabaseConnector,
) -> Result<PgPoolHandler, Box<dyn std::error::Error>> {
    let mut pg_config = Config::new();
    pg_config.host = Some(connector.database_host);
    pg_config.port = connector.database_port.parse::<u16>().ok();
    pg_config.dbname = Some(connector.database_name);
    pg_config.user = Some(connector.database_user);
    pg_config.password = Some(connector.database_pass);
    let pool = pg_config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(PgPoolHandler { pg_pool: pool })
}

async fn create_mssql_pool_connection(
    connector: DatabaseConnector,
) -> Result<MSSQLPoolHandler, Box<dyn std::error::Error>> {
    let mut config = tiberius::Config::new();
    config.host(connector.database_host);
    let port: u16 = connector.database_port.parse().unwrap_or(1433);
    config.port(port);
    config.authentication(AuthMethod::sql_server(
        connector.database_user,
        connector.database_pass,
    ));
    config.database(connector.database_name);
    config.trust_cert();

    let conn_manager = ConnectionManager::build(config).unwrap();
    let pool = bb8Pool::builder().max_size(10).build(conn_manager).await?;
    Ok(MSSQLPoolHandler { mssql_pool: pool })
}
