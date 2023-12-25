mod api;

use actix_web::{ web, App, HttpServer };
use api::student_api::create_student;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/student")
                .service(create_student)
        )
    })
    .bind(("127.0.0.1",4000))?
    .run()
    .await
}