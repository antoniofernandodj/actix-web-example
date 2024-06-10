use crate::api;
use actix_web::{web::{delete, get, post, put, scope}, Scope};


pub fn get_scope() -> Scope {

    scope("/users")

        .route(
            "/",
            post()
                    // .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::create)
        )

        .route(
            "/",
            get()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::get_all)
        )

        .route(
            "/{uuid}",
            get()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::get_one)
        )


        .route(
            "/{uuid}",
            delete()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::delete)
        )

        .route(
            "/{uuid}",
            put()
                    .guard(api::security::guards::JWTGuard)
                    .to(api::users::services::update)
        )

        .route(
            "/login",
            post()
                    .to(api::users::services::login)
        )

}