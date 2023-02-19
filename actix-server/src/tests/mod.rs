use dotenv::dotenv;

use crate::{init_pool, queries};
#[actix_web::test]
async fn fetch_complaints() {
    dotenv().ok();
    //here there are at least 2 queries in the table
    let q1 = queries::fetch_complaints(init_pool().await).await;
    let q2 = queries::fetch_complaints(init_pool().await).await;
    println!("{}", q1.len());
    println!("{}", q2.len());
    assert_eq!(1, q1.len());
    assert_eq!(2, q2.len());
    assert_eq!("Student injury", q1[0].category)
}
