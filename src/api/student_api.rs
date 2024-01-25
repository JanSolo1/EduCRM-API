use actix_web:: { post, get, patch, delete, web, Result, Responder, HttpResponse}; 
use serde_json::json;
use crate::db::helpers::*;
use crate::db::student::*;

#[post("/create")]
async fn create_student(student: web::Json<Student>) -> Result<impl Responder> {
    match insert_student(student.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }
}

#[get("/list")]
async fn students_list() -> Result<impl Responder> {
    let response = list_students().await; 
    match response {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }
}

#[get("/get/{student_id}")]
async fn get_student(student_id: web::Path<String>) -> Result<impl Responder> {
    let student_id = student_id.into_inner();
    println!("Getting info on strudent: {}", student_id);
    let response = get_student_by_id(student_id).await;
    match response {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }
}

#[patch("/update/{student_id}")]
async fn update_student(student_id: web::Path<String>, student: web::Json<Student>) -> Result<impl Responder> {
    let student_id = student_id.into_inner();
    println!("Updating Student: {}", student_id);
    match update_student_by_id(student_id, student.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }
}

#[delete("/delete/{student_id}")]
async fn delete_student(student_id: web::Path<String>) -> Result<impl Responder> {
    let student_id = student_id.into_inner();
    println!("Delete even on student: {}", student_id);
    match delete_student_by_id(student_id).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"message": "An error occurred", "error": e.to_string()}))),
    }
}