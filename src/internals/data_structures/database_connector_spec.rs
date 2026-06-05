use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use deadpool_postgres::Pool;
use tokio::net::TcpStream;
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
pub struct PgHandler{
    pub client: tokio_postgres::Client,
    pub _handler: tokio::task::JoinHandle<()>,
}

pub struct PgPoolHandler {
    pub pg_pool : Pool
}


pub struct MSSQLHandler{
    pub client : tiberius::Client<tokio_util::compat::Compat<TcpStream>>,
}

pub struct MSSQLPoolHandler{
    pub mssql_pool : Pool<ConnectionManager>
} 

//DB enums declaration:
pub enum DatabaseHandlers{
    Postgres(PgHandler),
    SqlServer(MSSQLHandler),
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