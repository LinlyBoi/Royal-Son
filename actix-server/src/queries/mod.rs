///////////////////////////////
//                           //
//        Fetching           //
//                           //
//       Complaints          //
//                           //
///////////////////////////////
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres, PgPool};
use actix_web::{post, HttpResponse, get, Responder, web};

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
pub async fn fetch_complaints(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let complaints = sqlx::query_as!(Complaint, "SELECT * FROM complaints LIMIT 100")
        .fetch_all(db_pool.get_ref())
        .await
        .expect("fetching complaint no works");

    HttpResponse::Ok().content_type("application/json").json(complaints)
}

pub async fn insert_complaint(inserted: Complaint, db_pool: Pool<Postgres>) {
    todo!()
}
