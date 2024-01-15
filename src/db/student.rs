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
    // let mut cursor = collection.find(None, None).await?;
    // while cursor.advance().await? {
    //     println!("{:?}", cursor.deserialize_current().unwrap());
    // }
    // print!("{:?}", cursor);

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

