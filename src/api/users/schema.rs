use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
#[allow(dead_code)]
pub struct PostExampleRequest {
    pub data: String
}


#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub msg: String,
}

impl CreateUserResponse {
    pub fn new(id: String, msg: String) -> Self {
        CreateUserResponse {id, msg}
    }
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}