use postgres::{Client, NoTls, Error};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let client = Client::connect("postgres:/postgres:postgres@localhost:5432/library", NoTls).await?;

    client.batch_execute ("
        CREATE TABLE IF NOT EXISTS author(
            id        SERIAL PRIMARY KEY,
            name      VARCHAR NOT NULL
            country   VARCHAR NOT NULL 
        )
    ")?;

    client.batch_execute ("
       CREATE TABLE IF NOT EXISTS book(
            id           SERIAL PRIMARY KEY
            title        VARCHAR NOT NULL
            author_id    INTEGER NOT NULL REFERENCES AUTHOR      
       )   
    ")?;

    Ok(())
}
