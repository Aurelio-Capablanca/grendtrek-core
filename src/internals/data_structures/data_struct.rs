use tokio::net::TcpStream;

pub struct DatabaseConnector {
    pub database_name: String,
    pub database_user: String,
    pub database_pass: String,
    pub database_host: String,
    pub database_port: String,
}


pub struct PgHandler{
    pub client: tokio_postgres::Client,
    pub _handler: tokio::task::JoinHandle<()>,
}

pub struct MSSQLHandler{
    pub client : tiberius::Client<tokio_util::compat::Compat<TcpStream>>,
}

pub enum DatabaseConnection{
    postgres(PgHandler),
    sql_server(MSSQLHandler),
    mysql(),
    None
}


pub struct DatabaseRegistry{
    origin: DatabaseConnection,
    destiny:DatabaseConnection
}