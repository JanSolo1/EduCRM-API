use actix_web:: { post, web, Result, Responder}; 
use futures_util::StreamExt as _;
use uuid::Uuid;
use serde::{ Serialize, Deserialize };
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Student {
    uuid: Option<String>,
    student_id: u32,
    student_fail: bool,
    student_name: String,
}

fn parse_payload(bytes: &[u8]) -> Result<Student, Error> {
    let student: Student = serde_json::from_slice(bytes)?;
    Ok(student)
}

#[post("/create")]
async fn create_student(mut body: web::Payload) -> Result<impl Responder> {
    let id = Uuid::new_v4().to_string();

    // we create a BytesMut (slice of memory to store bytes) which is the payload in our case
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let mut student = match parse_payload(&bytes) {
        Ok(student) => student,
        Err(e) => {
            println!("Failed to parse student: {}", e);
            return Err(actix_web::error::ErrorBadRequest("Invalid JSON"));
        },
    };

    student.uuid = Some(id);
    Ok(web::Json(student))
}