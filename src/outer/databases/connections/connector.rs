use std::io::Error;

use crate::internals::data_structures::database_connector_spec::{
    DatabaseConnector, DatabaseHandlers, MSSQLHandler, PgHandler, VendorOptions,
};
use deadpool_postgres::{Config, Pool};
use tiberius::{AuthMethod, Client};
use tokio::net::TcpStream;
use tokio_postgres::NoTls;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn generate_connections(
    connector: DatabaseConnector,
    vendor: VendorOptions,
) -> Result<DatabaseHandlers, Box<dyn std::error::Error>> {
    //Create
    let mut general_handler: DatabaseHandlers = match vendor {
        VendorOptions::POSTGRES => {
            let client_pg = create_posgres_conn(connector).await.unwrap();
            DatabaseHandlers::Postgres(client_pg)
        }
        VendorOptions::MSSQL => {
            let client_mssql = create_mssql_conn(connector).await.unwrap();
            DatabaseHandlers::SqlServer(client_mssql)
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
        DatabaseHandlers::Postgres(client_pg) => {
            let actions = client_pg.client.query("Select 1", &[]).await.unwrap();
            for action in actions {
                let row: i32 = action.get(0);
                println!("PostgreSQL Result {:?}", row);
            }
        }
        DatabaseHandlers::SqlServer(client_mssql) => {
            let sql_server_client = &mut client_mssql.client;
            let actions = sql_server_client
                .query("Select 1 as result", &[])
                .await
                .unwrap()
                .into_results()
                .await
                .unwrap();
            for action_s in actions {
                for action in action_s {
                    let result: i32 = action.get("result").unwrap();
                    println!("SQL Server Result : {:?}", result);
                }
            }
        }
        // DatabaseHandlers::MySql() => {
        //     println!("No active options yet!");
        // }
        DatabaseHandlers::None => {}
    }
    Ok(general_handler)
}

pub async fn create_posgres_conn(
    connector: DatabaseConnector,
) -> Result<PgHandler, Box<dyn std::error::Error>> {
    let name = connector.database_name;
    let user = connector.database_user;
    let pass = connector.database_pass;
    let host = connector.database_host;
    let port = connector.database_port;
    let literal = format!("postgres://{user}:{pass}@{host}:{port}/{name}");
    let (client, connection) = tokio_postgres::connect(literal.as_str(), NoTls)
        .await
        .unwrap();
    let handler = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprint!("connection error : {:?}", e)
        }
    });
    Ok(PgHandler {
        client: client,
        _handler: handler,
    })
}

pub fn create_postgres_pool_connection(
    connector: DatabaseConnector,
) -> Result<Pool, Box<dyn std::error::Error>> {
    let mut pg_config = Config::new();
    pg_config.host = Some(connector.database_host);
    pg_config.port = connector.database_port.parse::<u16>().ok();
    pg_config.dbname = Some(connector.database_name);
    pg_config.user = Some(connector.database_user);
    pg_config.password = Some(connector.database_pass);    
    let pool = pg_config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

pub async fn create_mssql_conn(
    connector: DatabaseConnector,
) -> Result<MSSQLHandler, Box<dyn std::error::Error>> {
    let mut configuration = tiberius::Config::new();
    configuration.host(connector.database_host);
    let port: u16 = match &connector.database_port.parse::<u16>() {
        Ok(val) => *val,
        Err(e) => {
            println!(
                "Not possible to convert    ! using default port! Error Message : {:?}",
                e
            );
            1433
        }
    };

    configuration.port(port);
    configuration.authentication(AuthMethod::sql_server(
        connector.database_user,
        connector.database_pass,
    ));
    configuration.database(connector.database_name);
    configuration.trust_cert();

    let tcp = TcpStream::connect(configuration.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = Client::connect(configuration, tcp.compat_write()).await?;

    Ok(MSSQLHandler { client: client })
}
