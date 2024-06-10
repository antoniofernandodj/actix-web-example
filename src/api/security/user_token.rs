use std::env;

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};


static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds


#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl TokenBodyResponse {
    pub fn new(token: String) -> Self {
        let token_type = String::from("Bearer");
        TokenBodyResponse{token, token_type}
    }
}

impl UserToken {
    pub fn generate_token(username: String, login_session: String) -> String {
        dotenv::dotenv().expect("Failed to read .env file");
        let max_age: i64 = match env::var("MAX_AGE") {
            Ok(val) => val.parse::<i64>().unwrap_or(ONE_WEEK),
            Err(_) => ONE_WEEK,
        };

        println!("Token Max Age: {}", max_age);

        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000 as i64; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + max_age,
            user: username.clone(),
            login_session: login_session.clone(),
        };

        let secret_key = env::var("SECRET_JWT").unwrap();

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&secret_key.as_bytes()),
        )
        .unwrap()
    }
}