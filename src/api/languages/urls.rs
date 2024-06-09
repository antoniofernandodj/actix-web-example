use actix_web::{Scope, web::scope};

pub fn get_scope() -> Scope {
    scope("/lang")
}