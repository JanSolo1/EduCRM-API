use mongodb::bson::doc;
use mongodb::bson;
use crate::db::helpers::*;
use crate::db::mdb::get_collection;
use dotenv::var;
use futures::stream::StreamExt;

pub async fn insert_student(student: Student, uuid: &str) -> Result<mongodb::results::InsertOneResult, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    let student_doc = doc! {
        "uuid": uuid.to_string(),
        "student_id": &student.student_id.to_string(),
        "student_first_name": &student.student_first_name,
        "student_middle_name": &student.student_middle_name,
        "student_last_name": &student.student_last_name,
        "student_medical_aid": &student.student_medical_aid,
        "student_doctor": &student.student_doctor,
        "student_doctor_phone": &student.student_doctor_phone,
        "student_gender": &student.student_gender,
        "student_dob": &student.student_dob,
        "student_race": &student.student_race,
        "student_phone": &student.student_phone,
        "student_email": &student.student_email,
        "student_citizenship": &student.student_citizenship,
        "student_home_language": &student.student_home_language,
        "student_resides_with": &student.student_resides_with,
        "student_disabilities": &student.student_disabilities,
        "student_learning_disabilities": &student.student_learning_disabilities,
        "student_father_figure_id_num": &student.student_father_figure_id_num,
        "student_father_figure_title": &student.student_father_figure_title,
        "student_father_figure_first_name": &student.student_father_figure_first_name,
        "student_father_figure_middle_name": &student.student_father_figure_middle_name,
        "student_father_figure_last_name": &student.student_father_figure_last_name,
        "student_father_figure_occupation": &student.student_father_figure_occupation,
        "student_father_figure_employer": &student.student_father_figure_employer,
        "student_father_figure_phone": &student.student_father_figure_phone,
        "student_father_figure_work_phone": &student.student_father_figure_work_phone,
        "student_father_figure_home_phone": &student.student_father_figure_home_phone,
        "student_father_figure_email": &student.student_father_figure_email,
        "student_father_figure_address": &student.student_father_figure_address,
        "student_mother_figure_id_num": &student.student_mother_figure_id_num,
        "student_mother_figure_title": &student.student_mother_figure_title,
        "student_mother_figure_first_name": &student.student_mother_figure_first_name,
        "student_mother_figure_middle_name": &student.student_mother_figure_middle_name,
        "student_mother_figure_last_name": &student.student_mother_figure_last_name,
        "student_mother_figure_occupation": &student.student_mother_figure_occupation,
        "student_mother_figure_employer": &student.student_mother_figure_employer,
        "student_mother_figure_phone": &student.student_mother_figure_phone,
        "student_mother_figure_work_phone": &student.student_mother_figure_work_phone,
        "student_mother_figure_home_phone": &student.student_mother_figure_home_phone,
        "student_mother_figure_email": &student.student_mother_figure_email,
        "student_mother_figure_address": &student.student_mother_figure_address,
    };

    let student_struct: Student = bson::from_bson(bson::Bson::Document(student_doc.clone())).unwrap();

    let res = collection.insert_one(&student_struct, None).await?;
    println!("inserted document with id {}", res.inserted_id);

    Ok(res)
}

pub async fn list_students() -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    let mut cursor = collection.find(None, None).await?;
    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(e) => return Err(e.into()),
        }
    }


    Ok(results)
}

pub async fn get_student_by_uuid(uuid: String) -> Result<Option<Student>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    let filter = doc! { "uuid": uuid };
    let result = collection.find_one(filter, None).await?;

    Ok(result)
}

pub async fn update_student_in_db(student: Student) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    println!("before cleaning data");
    let filter = doc! { "uuid": &student.uuid };
    let update = doc! { 
        "$set": 
            { 
                "student_first_name": &student.student_first_name, 
            } 
        };
    println!("before update");
    let result = collection.update_one(filter, update, None).await?;
    println!("before update");
    Ok(())
}