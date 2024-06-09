use actix_web::HttpResponse;
use actix_web::web::{Path, Json};
use entity::user::Model as User;
use entity::user::ActiveModel;

use sea_orm::DeleteResult;
// use crate::state::AppState;
use sea_orm::{prelude::DatabaseConnection, InsertResult, prelude::DbErr};
use serde_json::json;
use crate::api::users::schema::{CreateUserRequest, CreateUserResponse, UpdateUserRequest};
use crate::repositories::user::{UserRepository, Repository};
use crate::database;
use crate::errors;
use crate::prelude::*;


pub async fn create(
    body: Json<CreateUserRequest>
) -> Result<HttpResponse, errors::HTTP400Error> {

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);
    let body: CreateUserRequest = body.into_inner();

    let result: Result<InsertResult<ActiveModel>, DbErr> = repo.create_user(
        body.username,
        body.password_hash,
        body.name,
        body.email
    ).await;

    if let Ok(insert) = result {

        connection.close().await.unwrap();
        let body: CreateUserResponse = CreateUserResponse::new(
            insert.last_insert_id,
            "Usuario criado com sucesso!".to_owned()
        );
        let response: HttpResponse = json_response(body, Some(201));
        Ok(response)

    } else {
        let msg: String = format!("Error Creating the user!");
        Err(errors::HTTP400Error { name: msg })
    }
}


pub async fn get_one(
    path: Path<String>
) -> Result<HttpResponse, errors::HTTP404Error> {

    let uuid: String = path.into_inner();

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Option<User> = repo.find_by_uuid(uuid.clone()).await;

    if let Some(user) = result {

        connection.close().await.unwrap();

        let response: HttpResponse = json_response(user, None);
        Ok(response)

    } else {
        let msg: String = format!("User {} not found!", uuid);
        Err(errors::HTTP404Error { name: msg })
    }
}


pub async fn get_all() -> Result<HttpResponse, errors::HTTP404Error> {

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Vec<User> = repo.get_many().await;

    connection.close().await.unwrap();

    let response: HttpResponse = json_response(result, None);
    Ok(response)

}


pub async fn delete(
    path: Path<String>
) -> Result<HttpResponse, errors::HTTP400Error> {

    let uuid: String = path.into_inner();

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Option<User> = repo.find_by_uuid(uuid.clone()).await;

    if result.is_none() {
        connection.close().await.unwrap();
        let msg: String = format!("User {} not found!", uuid);
        return Err(errors::HTTP400Error { name: msg })
    }

    let result: Result<DeleteResult, DbErr> = repo.delete_by_uuid(uuid.clone()).await;

    if result.is_ok() {
        connection.close().await.unwrap();
        let msg: String = format!("Usuário {} deletado com sucesso!", uuid);

        let body: sea_orm::prelude::Json = json!({ "message": msg });
        return Ok(HttpResponse::Ok().json(body))
    } else {
        let msg: String = format!("Erro ao deletar o usuário {}!", uuid);
        return Err(errors::HTTP400Error { name: msg })
    }

}


pub async fn update(
    path: Path<String>,
    body: Json<UpdateUserRequest>
) -> Result<HttpResponse, errors::HTTP400Error> {

    let uuid: String = path.into_inner();
    let update_data: UpdateUserRequest = body.into_inner();

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Option<User> = repo.find_by_uuid(uuid.clone()).await;

    if result.is_none() {
        connection.close().await.unwrap();
        let msg: String = format!("User {} not found!", uuid);
        return Err(errors::HTTP400Error { name: msg })
    }

    let updated_user_result: Result<User, DbErr> = repo.update_user(
        uuid.clone(),
        update_data.username,
        update_data.password_hash,
        update_data.name,
        update_data.email
    ).await;

    match updated_user_result {
        Ok(updated_user) => {
            connection.close().await.unwrap();
            let response: HttpResponse = json_response(updated_user, Some(200));
            Ok(response)
        },
        Err(_) => {
            connection.close().await.unwrap();
            let msg: String = format!("Erro ao atualizar o usuário {}!", uuid);
            Err(errors::HTTP400Error { name: msg })
        }
    }
}