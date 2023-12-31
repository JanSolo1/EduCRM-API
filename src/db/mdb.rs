use mongodb::{ bson::doc, Client};
use actix_web::web;
use crate::db::student::Student;

pub async fn add_student(student: web::Json<Student>) {
    // Connect to MongoDB
    let uri = "mongodb://localhost:27017";
    let client = match Client::with_uri_str(uri).await {
        Ok(client) => client,
        Err(e) => panic!("Error establishing connection to MongoDB: {}", e),
    };

    // Access the "users" collection
    let collection = client.database("edu_crm_db").collection("datalake_dev_student");
    // Create a BSON document from the User struct

    // please shcange the belowe code to match the struct please
    let student_doc = doc! {
        "uuid": &student.uuid,
        "student_id": &student.student_id,
        "student_first_name": &student.student_first_name,
        "student_middle_name": &student.student_middle_name,
        "student_last_name": &student.student_last_name,
        "student_dob": &student.student_dob,
        "student_email": &student.student_email,
        "student_phone": &student.student_phone,
        "student_gender": &student.student_gender,
    };

    // Insert the document into the "users" collection
    let res = match collection.insert_one(student_doc, None).await {
        Ok(res) => res,
        Err(e) => panic!("Error inserting document: {}", e),
    };
    println!("inserted document with id {}", res.inserted_id);


    // Return the JSON data back to the client
}