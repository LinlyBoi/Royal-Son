use actix_web::post;
///////////////////////////////
//                           //
//        Fetching           //
//                           //
//       Complaints          //
//                           //
///////////////////////////////
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

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

pub async fn fetch_complaints(limit: i64, db_pool: Pool<Postgres>) -> Vec<Complaint> {
    let complaints = sqlx::query_as!(Complaint, "SELECT * FROM complaints LIMIT $1", limit)
        .fetch_all(&db_pool)
        .await
        .expect("fetching complaint no works");

    complaints
}

pub async fn insert_complaint(inserted: Complaint, db_pool: Pool<Postgres>) {
    let record = sqlx::query!(r#"INSERT INTO complaints ()"#)
}
