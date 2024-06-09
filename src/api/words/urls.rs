use actix_web::Scope;
use actix_web::web::{scope, get, post};
use super::services;
use crate::api::security::guards;

pub fn get_scope() -> Scope {
    scope("/words")
        .route(
            "/{id}",
            get()
                    .to(services::get_word)
        )
        .route(
            "/",
            post()
                    .to(services::create_word)
        )
        .route(
            "/",
            get()
                    .guard(guards::LoggedInGuard)
                    .to(services::get_words)
        )
}
