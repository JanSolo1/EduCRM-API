use actix_web:: { web, HttpResponse, Scope };
use chrono:: { Utc, Duration };

pub fn user_scope() -> Scope {
  web::scope("/user")
    .route("/encode-token/{id}", web::get().to(encode_token))
    .route("decode-token", web::post().to(decode_token))
    .route("/protected", web::get().to(protected))
}

// exp is the experation time
#[derive(Serialize, Deserialize)]
struct Claims {
  id: usize,
  exp: usize,
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
  let id: usize = path.into_inner();
  let exp: usize = (Utc::now() + Duration::days(1)).timestamp() as usize;
  let claims: Claims = Claims { id, };
  HttpResponse::Ok().json(
    Response {
      message: "encode_token".to_owned()
    }
  )
}

async fn decode_token() -> HttpResponse {
  HttpResponse::Ok().body("decode token")
}

async fn protected() -> HttpResponse {
  HttpResponse::Ok().body("protected")
}
