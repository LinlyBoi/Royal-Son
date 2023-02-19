use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

mod queries;
#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let (port, address) = init_address();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(init_pool()))
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}

fn init_address() -> (u16, String) {
    let port: u16 = env::var("ACTIX_PORT")
        .expect("SET ACTIX_PORT PLOX")
        .parse()
        .expect("NOPE CANT PARSE THIS WHAT DID YOU PUT IN?!!!");
    let address = env::var("ACTIX_IP").expect("SET ACTIX_IP PLOX");
    (port, address)
}

//AAA sync -> Database pools YEAA
async fn init_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("Put a DB url in the .env file dumbass");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("No pool connection man :(")
}
