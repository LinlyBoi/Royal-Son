use dotenv::dotenv;

use crate::{init_pool, queries};
#[actix_web::test]
async fn fetch_complaints() {
    dotenv().ok();
    //here there are at least 2 queries in the table
}
