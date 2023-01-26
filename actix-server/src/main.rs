use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[cfg(test)]
mod tests;

pub mod pog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port: u16 = env::var("ACTIX_PORT")
        .expect("SET ACTIX_PORT PLOX")
        .parse()
        .expect("NOPE CANT PARSE THIS WHAT DID YOU PUT IN?!!!");
    let address = env::var("ACTIX_IP").expect("SET ACTIX_IP PLOX");
    let database_url = env::var("DATABASE_URL").expect("Put a DB url in the .env file dumbass");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("No pool connection man :(");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
