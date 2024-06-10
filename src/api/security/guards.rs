use std::env;

use actix_web::{guard::{Guard, GuardContext}, http::header::HeaderValue};
use jsonwebtoken::{DecodingKey, TokenData, Validation};
// use sea_orm::DatabaseConnection;
// use crate::repositories::user::UserRepository;
// use entity::user::Model as User;
use super::user_token::UserToken;

pub struct JWTGuard;
impl Guard for JWTGuard {
    fn check(&self, ctx: &GuardContext) -> bool {

        let headers = ctx.head().headers();

        let header_option1 = headers.get("Authorization");
        let header_option2 = headers.get("authorization");

        if let Some(v) = header_option1 {

            return parse_header_value(v)

        } else if let Some(v) = header_option2 {

            return parse_header_value(v)

        } else {
            return false
        }
    }
}

fn parse_header_value(header_value: &HeaderValue) -> bool {
    let str_result = header_value.to_str();

    if let Ok(str_value) = str_result {

        if str_value.contains("Bearer") || str_value.contains("bearer") {
            let token = str_value[6..str_value.len()].trim();
    
            if let Ok(_) = decode_token(token.to_string()) {
                return true;
            } else {
                return false
            }
        }
    }

    return false
}

fn decode_token(
    token: String
) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    
    let secret_key = env::var("SECRET_JWT").unwrap();
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&secret_key.as_bytes()),
        &Validation::default(),
    )
}


// fn verify_token(
//     token_data: &TokenData<user_token::UserToken>
// ) -> Result<String, String> {

//     if is_valid_login_session(&token_data.claims) {
//         Ok(token_data.claims.user.to_string())
//     } else {
//         Err("Invalid token".to_string())
//     }
// }

// fn is_valid_login_session(user_token: &user_token::UserToken) -> bool {
//     println!("user_token: {:?}", user_token);
//     return true

//     users
//         .filter(username.eq(&user_token.user))
//         .filter(login_session.eq(&user_token.login_session))
//         .get_result::<User>(conn)
//         .is_ok()
// }