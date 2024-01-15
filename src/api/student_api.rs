// use actix_web::web::Query;
use actix_web:: { post, get, web, Result, Responder, HttpResponse}; 
// use futures_util::io::empty;
use uuid::Uuid;
use serde_json::json;
use crate::db::helpers::*;
use crate::db::student::*;

#[post("/create")]
async fn create_student(student: web::Json<Student>) -> Result<impl Responder> {
    let uuid = Uuid::new_v4().to_string();

    match insert_student(student.into_inner(), &uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"message": "Student created successfully", "uuid": uuid}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }

}

#[get("/get")]
async fn get_student(student_search: web::Query<StudentSearch>) -> Result<impl Responder> {
    let student_search = student_search.into_inner();

    if student_search.uuid.is_none() {
        let response = list_students().await; 
        match response {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
        }
    } else {
        return Ok(HttpResponse::BadRequest().json(json!({"message": "uuid is required"})));
    }
}