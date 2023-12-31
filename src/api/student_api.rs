use actix_web:: { post, web, Result, Responder, HttpResponse}; 
use uuid::Uuid;
use serde_json::json;
use crate::db::student::Student;
use crate::db::mdb::add_student;

#[post("/create")]
async fn create_student(student: web::Json<Student>) -> Result<impl Responder> {
    let uuid = Uuid::new_v4().to_string();

    let student_json =json!({
        "uuid": uuid.to_string(),
        "student_id": &student.student_id,
        "student_first_name": &student.student_first_name,
        "student_middle_name": &student.student_middle_name,
        "student_last_name": &student.student_last_name,
        "student_dob": &student.student_dob,
        "student_email": &student.student_email,
        "student_phone": &student.student_phone.to_string(),
        "student_gender": &student.student_gender,
    });

    let student_struct: Student = serde_json::from_value(student_json.clone()).unwrap();

    add_student(web::Json(student_struct)).await;

    Ok(HttpResponse::Ok().json(student_json))
}