mod api;
mod state;
mod prelude;
mod errors;
mod settings;
mod database;
mod repositories;

use actix_web::{
    Scope,
    HttpRequest,
    Result,
};
use errors::ServiceError;
use actix_web::web::{
    ServiceConfig,
    route,
    Data
};
use state::AppState;
use shuttle_actix_web::ShuttleActixWeb;


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    
    settings::init();
    database::init().await;

    let config = move |cfg: &mut ServiceConfig| {
        
        let app_data: Data<AppState> = state::get_app_data();
        let users_scope: Scope = api::users::urls::get_scope();
        let words_scope: Scope = api::words::urls::get_scope();
        let languages_scope: Scope = api::languages::urls::get_scope();
    
        cfg
        .app_data(app_data)
        .service(words_scope)
        .service(users_scope)
        .service(languages_scope)
        .default_service(route().to(not_found));
    };

    Ok(config.into())
}

async fn not_found(request: HttpRequest) -> Result<String, ServiceError> {
    let path: &str = request.path();
    let msg: String = format!("Path {} not found!", path);
    Err(ServiceError::NotFound { error_message: msg })
}