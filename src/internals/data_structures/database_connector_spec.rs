use bb8::Pool as bb8Pool;
use bb8_tiberius::ConnectionManager;
use deadpool_postgres::Pool as dpPool;
//use tokio::net::TcpStream;
use serde::Deserialize;

//connector metadata: 
pub struct DatabaseConnector {
    pub database_name: String,
    pub database_user: String,
    pub database_pass: String,
    pub database_host: String,
    pub database_port: String,
}

//Handlers:
// pub struct PgHandler{
//     pub client: tokio_postgres::Client,
//     pub _handler: tokio::task::JoinHandle<()>,
// }

pub struct PgPoolHandler {
    pub pg_pool : dpPool
}


// pub struct MSSQLHandler{
//     pub client : tiberius::Client<tokio_util::compat::Compat<TcpStream>>,
// }

pub struct MSSQLPoolHandler{
    pub mssql_pool : bb8Pool<ConnectionManager>
} 

//DB enums declaration:
pub enum DatabaseHandlers{
   // Postgres(PgHandler),
    PostgresPool(PgPoolHandler),
    //SqlServer(MSSQLHandler),
    SqlServerPool(MSSQLPoolHandler),
    //_MySql(),
    None
}

//Vendors
#[derive(Debug, Deserialize)]
pub enum VendorOptions{
    POSTGRES,
    MSSQL,
    MYSQL,
    SQLITE,
    NONE
}