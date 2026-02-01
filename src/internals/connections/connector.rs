use tiberius::{AuthMethod, Client};
use tokio::net::TcpStream;
use tokio_postgres::{NoTls};
use tokio_util::compat::TokioAsyncWriteCompatExt;
use crate::internals::data_structures::data_struct::{MSSQLHandler, PgHandler};



pub async fn create_posgres_conn() -> Result<PgHandler, Box<dyn std::error::Error>> {
    let literal = "postgres://superuserp:jkl555@localhost:5432/transcontinentalshippings";
    let (client, connection) = tokio_postgres::connect(literal, NoTls).await.unwrap();

    let handler = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprint!("connection error : {:?}", e)
        }
    });        
    Ok(PgHandler{
        client: client,
        _handler: handler,
    })
}


pub async fn create_mssql_conn() -> Result<MSSQLHandler, Box<dyn std::error::Error>> {
    let mut configuration = tiberius::Config::new();
    configuration.host("localhost");
    configuration.port(1433);
    configuration.authentication(AuthMethod::sql_server("sa", "jklgHnbvc555SS"));
    configuration.database("AdventureWorks2022");
    configuration.trust_cert();
    
    let tcp = TcpStream::connect(configuration.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = Client::connect(configuration, tcp.compat_write()).await?;
    
    Ok(MSSQLHandler { client: client })
}
