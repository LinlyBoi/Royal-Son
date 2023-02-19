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
use sqlx::{PgPool, Postgres, Pool};

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

pub async fn fetch_complaints(db_pool: Pool<Postgres>) -> Vec<Complaint> {
    let complaints = sqlx::query_as!(Complaint, "SELECT * FROM complaints")
        .fetch_all(&db_pool)
        .await
        .expect("fetching complaint no works");
    complaints
}

pub async fn complaints_filter(db_pool: Pool<Postgres>, column: String, column_val: String) -> () {
    let query = format!("SELECT * FROM complaints WHERE {} = $1", column);
    let comp_vec = sqlx::query_as!(Complaint, query.to_str(), column_vals).fetch_all(&db_pool).await.expect("Filter succ");

}
