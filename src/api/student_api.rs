use actix_web:: { post, get, web, Result, Responder, HttpResponse}; 
use uuid::Uuid;
use serde_json::json;
use mongodb::bson::doc;
use mongodb::bson;
use crate::db::helpers::Student;
use crate::db::mdb::get_collection;

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

    let collection = get_collection("edu_crm_db", "datalake_dev_student").await;

    let student_doc = doc! {
        "uuid": uuid.to_string(),
        "student_id": &student.student_id,
        "student_first_name": &student.student_first_name,
        "student_middle_name": &student.student_middle_name,
        "student_last_name": &student.student_last_name,
        "student_dob": &student.student_dob,
        "student_email": &student.student_email,
        "student_phone": &student.student_phone,
        "student_gender": &student.student_gender,
    };
    
    let student_struct: Student = bson::from_bson(bson::Bson::Document(student_doc.clone())).unwrap();

    let res = match collection.insert_one(&student_struct, None).await {
        Ok(res) => res,
        Err(e) => panic!("Error inserting document: {}", e),
    };
    println!("inserted document with id {}", res.inserted_id);

    Ok(HttpResponse::Ok().json(student_json))
}

// #[get("/get")]
// async fn get_student() -> Result<impl Responder> {

//     student
//     Ok(HttpResponse::Ok().json(json!({"message": "get student"})))
// }