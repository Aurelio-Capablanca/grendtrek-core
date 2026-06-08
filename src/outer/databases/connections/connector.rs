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
        DatabaseHandlers::Postgres(client_pg) => {
            let actions = client_pg.client.query("Select 1", &[]).await.unwrap();
            for action in actions {
                let row: i32 = action.get(0);
                println!("PostgreSQL Result {:?}", row);
            }
        }
        DatabaseHandlers::PostgresPool(pool_pg) => {
            let client = pool_pg.pg_pool.get().await.unwrap();
            let actions = client.query("SELECT 1", &[]).await.unwrap();
            for action in actions {
                let row: i32 = action.get(0);
                println!("Pool PG Result : {:?}", row);
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
        // DatabaseHandlers::MySql() => {
        //     println!("No active options yet!");
        // }
        DatabaseHandlers::None => {}
    }
    Ok(general_handler)
}

// pub async fn create_posgres_conn(
//     connector: DatabaseConnector,
// ) -> Result<PgHandler, Box<dyn std::error::Error>> {
//     let name = connector.database_name;
//     let user = connector.database_user;
//     let pass = connector.database_pass;
//     let host = connector.database_host;
//     let port = connector.database_port;
//     let literal = format!("postgres://{user}:{pass}@{host}:{port}/{name}");
//     let (client, connection) = tokio_postgres::connect(literal.as_str(), NoTls)
//         .await
//         .unwrap();
//     let handler = tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprint!("connection error : {:?}", e)
//         }
//     });
//     Ok(PgHandler {
//         client: client,
//         _handler: handler,
//     })
// }

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

// pub async fn create_mssql_conn(
//     connector: DatabaseConnector,
// ) -> Result<MSSQLHandler, Box<dyn std::error::Error>> {
//     let mut configuration = tiberius::Config::new();
//     configuration.host(connector.database_host);
//     let port: u16 = match &connector.database_port.parse::<u16>() {
//         Ok(val) => *val,
//         Err(e) => {
//             println!(
//                 "Not possible to convert    ! using default port! Error Message : {:?}",
//                 e
//             );
//             1433
//         }
//     };

//     configuration.port(port);
//     configuration.authentication(AuthMethod::sql_server(
//         connector.database_user,
//         connector.database_pass,
//     ));
//     configuration.database(connector.database_name);
//     configuration.trust_cert();

//     let tcp = TcpStream::connect(configuration.get_addr()).await?;
//     tcp.set_nodelay(true)?;
//     let client = Client::connect(configuration, tcp.compat_write()).await?;

//     Ok(MSSQLHandler { client: client })
// }

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
