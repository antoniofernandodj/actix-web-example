mod api;
mod state;
mod prelude;
mod errors;
mod settings;
mod database;
mod repositories;

// use entity;


// use std::env;
// use sea_orm::{ConnectionTrait, Database, DbErr, Statement};
// use futures::executor::block_on;

use actix_web::{
    Scope,
    HttpRequest,
    Result,
};
use actix_web::web::{
    ServiceConfig,
    route,
    Data
};
use state::AppState;
use shuttle_actix_web::ShuttleActixWeb;


// async fn run() -> Result<(), DbErr> {

//     let database_url = env::var("DATABASE_URL").unwrap();
//     let db_name = env::var("DB_NAME").unwrap();

//     let db = Database::connect(database_url).await?;
//     let stmt = format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name);
//     let db_backend = db.get_database_backend();
//     db.execute(Statement::from_string(db_backend, stmt)).await?;

    
//     Ok(())
// }


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

async fn not_found(request: HttpRequest) -> Result<String, errors::HTTP404Error> {
    let path: &str = request.path();
    let msg: String = format!("Path {} not found!", path);
    Err(errors::HTTP404Error { name: msg })
}