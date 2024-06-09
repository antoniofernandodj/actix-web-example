use crate::api;
use actix_web::{web::{delete, get, post, put, scope}, Scope};


pub fn get_scope() -> Scope {

    scope("/users")

        .route(
            "/",
            post().to(api::users::services::create)
        )

        .route(
            "/",
            get().to(api::users::services::get_all)
        )

        .route(
            "/{uuid}",
            get().to(api::users::services::get_one)
        )


        .route(
            "/{uuid}",
            delete().to(api::users::services::delete)
        )

        .route(
            "/{uuid}",
            put().to(api::users::services::update)
        )

}