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
use sqlx::{query_as, PgPool};

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

pub async fn fetch_complaints(db_pool: &PgPool) -> Vec<Complaint> {
    let complaints = sqlx::query_as!(Complaint, "SELECT * FROM complaints")
        .fetch_all(db_pool)
        .await
        .expect("fetching complaint no works");
    complaints
}

pub async fn complaints_filter(
    db_pool: &PgPool,
    column_name: String,
    column_val: String,
) -> Vec<Complaint> {
    let complaints = query_as!(
        Complaint,
        r#"SELECT * FROM complaints WHERE $1 = $2"#,
        column_name,
        column_val
    )
    .fetch_all(db_pool)
    .await
    .expect("Couldn't fetch filtered complaints");
    complaints
}
pub async fn complaint_before_date(
    db_pool: &PgPool,
    desired_date: NaiveDateTime,
) -> Vec<Complaint> {
    let complaints = query_as!(
        Complaint,
        r#"SELECT * FROM complaints WHERE write_date < $1"#,
        desired_date,
    )
    .fetch_all(db_pool)
    .await
    .expect("Error fetching complaints from before a date");
    complaints
}

//json function
#[get("/api/json/complaints")]
pub async fn json_complaints(db_pool: web::Data<PgPool>) -> impl Responder {
    let complaints = fetch_complaints(db_pool.get_ref()).await;
    HttpResponse::Ok()
        .content_type("application/json")
        .json(complaints)
}

#[get("/api/json/complaints/filter/{column_name}/{column_value}")]
pub async fn json_filter_complaints(
    db_pool: web::Data<PgPool>,
    column_values: web::Path<(String, String)>,
) -> impl Responder {
    let column_values = column_values.into_inner();
    let complaints = complaints_filter(db_pool.get_ref(), column_values.0, column_values.1).await;
    HttpResponse::Ok()
        .content_type("application/json")
        .json(complaints)
}

// #[get("/api/json/complaints/filter/before_date")]
// pub async fn json_complaints_before_date(
//     db_pool: web::Data<PgPool>,
//     desired_date: web::Query<String>,
// ) -> impl Responder {
// }
