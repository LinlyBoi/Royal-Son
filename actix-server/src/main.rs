use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod queries;
#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let (port, address) = init_address();

    let database_url = env::var("DATABASE_URL").expect("Put a DB url in the .env file dumbass");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("No pool connection man :(");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(queries::json_complaints)
            .service(queries::json_filter_complaints)
            // .service(queries::json_complaints_before_date)
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
        .expect("Couldn't parse the ACTIX_PORT variable >:(");
    let address = env::var("ACTIX_IP").expect("SET ACTIX_IP PLEASE");
    (port, address)
}
