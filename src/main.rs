mod api;
mod db;
mod scopes;

use actix_web::{ web, App, HttpServer };
use api::student_api::*;
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(String::from("secret")))
            .service(user_scope())
            .service(
                web::scope("/student")
                    .service(create_student)
                    .service(students_list)
                    .service(get_student)
                    .service(update_student)
                    .service(delete_student)
        )
    })
    .bind(("127.0.0.1",4000))?
    .run()
    .await
}
