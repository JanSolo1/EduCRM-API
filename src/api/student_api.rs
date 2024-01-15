use actix_web:: { post, get, web, Result, Responder, HttpResponse}; 
use uuid::Uuid;
use serde_json::json;
use crate::db::helpers::*;
use crate::db::student::insert_student;

#[post("/create")]
async fn create_student(student: web::Json<Student>) -> Result<impl Responder> {
    let uuid = Uuid::new_v4().to_string();

    match insert_student(student.into_inner(), &uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"message": "Student created successfully", "uuid": uuid}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }

}

// #[get("/get")]
// async fn get_student(student_search: web::Query<StudentSearch>) -> Result<impl Responder> {
    // let student_search = student_search.into_inner();
    // Ok(HttpResponse::Ok().json(json!({"message": "get student"})))
// }