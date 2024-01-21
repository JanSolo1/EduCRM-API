use mongodb::bson::{doc, to_document};
use mongodb::bson;
use crate::db::helpers::*;
use crate::db::mdb::get_collection;
use dotenv::var;
use futures::stream::StreamExt;

pub async fn insert_student(student: Student) -> Result<mongodb::results::InsertOneResult, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    let student_doc = to_document(&student)?;
    let student_struct: Student = bson::from_bson(bson::Bson::Document(student_doc.clone())).unwrap();
    let res = collection.insert_one(&student_struct, None).await?;

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

pub async fn get_student_by_id(student_id: String) -> Result<Option<Student>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    let filter = doc! { "student_id": student_id };
    let result = collection.find_one(filter, None).await?;

    Ok(result)
}

pub async fn update_student_by_id(student: Student) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let collection = get_collection(
        &var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"), 
        &var("MONGODB_DB_STUDENTS_COLLECTION").expect("MONGODB_DB_STUDENTS_COLLECTION must be set"))
        .await;

    let student_doc = to_document(&student)?;

    let filter = doc! { "student_id": &student.student_id };
    let update = doc! { "$set": student_doc};

    let result = collection.update_one(filter, update, None).await?;

    Ok(())
}