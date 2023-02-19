///////////////////////////////
//                           //
//        Fetching           //
//                           //
//       Complaints          //
//                           //
///////////////////////////////
use actix_web::{get, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Complaint {
    complaint_id: String,
    student_id: String,
    department: String,
    status: String,
    pub category: String,
    pub subject: String,
    pub body: String,
    pub write_date: NaiveDateTime,
}

#[get("/api/json/complaints")]
pub async fn fetch_complaints(db_pool: web::Data<PgPool>) -> impl Responder {
    let complaints = sqlx::query_as!(Complaint, "SELECT * FROM complaints")
        .fetch_all(db_pool.get_ref())
        .await
        .expect("fetching complaint no works");
    println!("balls");

    HttpResponse::Ok()
        .content_type("application/json")
        .json(complaints)
}

// pub async fn insert_complaint(inserted: Complaint, db_pool: Pool<Postgres>) {
//     todo!()
// }
