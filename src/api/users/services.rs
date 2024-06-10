use actix_web::HttpResponse;
use actix_web::web::{Json, Path};
use bcrypt::{hash, verify, DEFAULT_COST};
use entity::user::Model as User;
use entity::user::ActiveModel;

use sea_orm::DeleteResult;
// use crate::state::AppState;
use sea_orm::{prelude::DatabaseConnection, InsertResult, prelude::DbErr};
use serde_json::json;
use uuid::Uuid;
use crate::api::security::user_token::UserToken;
use crate::api::security::user_token::TokenBodyResponse;
use crate::api::users::schema::{
    CreateUserRequest,
    CreateUserResponse,
    UpdateUserRequest,
    LoginDTO
};
use crate::repositories::user::UserRepository;
use crate::database;
use crate::errors::ServiceError;
use crate::prelude::*;


pub async fn create(
    body: Json<CreateUserRequest>
) -> Result<HttpResponse, ServiceError> {

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);
    let body: CreateUserRequest = body.into_inner();

    let results: [(Option<User>, &str); 2] = [
        (repo.find_by_username(&body.username).await, "Username"),
        (repo.find_by_email(&body.email).await, "Email")
    ];

    for (result, field) in results.iter() {
        if let Some(_) = result {
    
            connection.close().await.unwrap();
    
            return Err(
                ServiceError::InternalServerError {
                    error_message: format!("Error Creating the user! {} taken.", field)
                }
            )    
        }
    }

    let password_hash = hash(&body.password_hash, DEFAULT_COST).unwrap();

    let result: Result<InsertResult<ActiveModel>, DbErr> = repo.create_user(
        body.username,
        password_hash,
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
        Err(ServiceError::InternalServerError { error_message: msg })
    }
}


pub async fn get_one(
    path: Path<String>
) -> Result<HttpResponse, ServiceError> {

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
        Err(ServiceError::NotFound { error_message: msg })
    }
}


pub async fn get_all() -> Result<HttpResponse, ServiceError> {

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Vec<User> = repo.get_many().await;

    connection.close().await.unwrap();

    let response: HttpResponse = json_response(result, None);
    Ok(response)

}


pub async fn delete(
    path: Path<String>
) -> Result<HttpResponse, ServiceError> {

    let uuid: String = path.into_inner();

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Option<User> = repo.find_by_uuid(uuid.clone()).await;

    if result.is_none() {
        connection.close().await.unwrap();
        let msg: String = format!("User {} not found!", uuid);
        return Err(ServiceError::NotFound { error_message: msg })
    }

    let result: Result<DeleteResult, DbErr> = repo.delete_by_uuid(uuid.clone()).await;

    if result.is_ok() {
        connection.close().await.unwrap();
        let msg: String = format!("Usuário {} deletado com sucesso!", uuid);

        let body: sea_orm::prelude::Json = json!({ "message": msg });
        return Ok(HttpResponse::Ok().json(body))
    } else {
        let msg: String = format!("Erro ao deletar o usuário {}!", uuid);
        return Err(ServiceError::InternalServerError { error_message: msg })
    }

}


pub async fn update(
    path: Path<String>,
    body: Json<UpdateUserRequest>
) -> Result<HttpResponse, ServiceError> {

    let uuid: String = path.into_inner();
    let update_data: UpdateUserRequest = body.into_inner();

    let connection: DatabaseConnection = database::init().await;
    let repo: UserRepository = UserRepository::new(&connection);

    let result: Option<User> = repo.find_by_uuid(uuid.clone()).await;

    if result.is_none() {
        connection.close().await.unwrap();
        let msg: String = format!("User {} not found!", uuid);
        return Err(ServiceError::NotFound { error_message: msg })
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
            Err(ServiceError::BadRequest { error_message: msg })
        }
    }
}



// POST users/login
pub async fn login(
    body: Json<LoginDTO>
) -> Result<HttpResponse, ServiceError> {
    let connection: DatabaseConnection = database::init().await;

    let auth_service = AuthService::new(&connection);

    let logged_user = auth_service.login_user(
        &body.login,
        &body.password
    ).await;

    match logged_user {

        None => {
            connection.close().await.unwrap();

            Err(ServiceError::Unauthorized {
                error_message: "MESSAGE_USER_NOT_FOUND".to_string(),
            })
        },

        Some(logged_user) => {

            let login_session = AuthService::generate_login_session();
            let token: String = UserToken::generate_token(
                logged_user.uuid.clone(), login_session
            );

            connection.close().await.unwrap();
            let json = TokenBodyResponse::new(token);
            Ok(json_response(json, Some(200)))

        },
    }
}


struct AuthService<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> AuthService<'a> {
    fn new<'b>(connection: &'a DatabaseConnection) -> Self {
        AuthService {
            connection:connection,
        }
    }

    pub async fn login_user(self, login: &str, password: &str) -> Option<User> {

        let repo = UserRepository::new(self.connection);
    
        let users: Vec<User> = repo.get_many().await;
    
        let mut users_filtred: Vec<User> = users.clone().into_iter()
            .filter(|user| user.email == login)
            .collect();
    
        let username_iter: Vec<User>  = users.into_iter()
            .filter(|user| user.username == login)
            .collect();
    
        users_filtred.extend(username_iter);
    
        if users_filtred.is_empty() {
            return None
        }
    
        let user_to_verify = users_filtred[0].clone();
    
        if !user_to_verify.password_hash.is_empty()
            && verify(password, &user_to_verify.password_hash).unwrap() {
    
            return Some(user_to_verify)
        } else {
            return None
        }
    
    }
    
    
    fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }
    
    
}

