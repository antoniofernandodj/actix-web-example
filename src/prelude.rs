use serde::Serialize;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

pub fn json_response<T:Serialize>(response: T, status_code: Option<u16>) -> HttpResponse {
    if let Some(v) = status_code {
        HttpResponse::build(StatusCode::from_u16(v).unwrap()).json(response)
    } else {
        HttpResponse::Ok().json(response)
    }
}