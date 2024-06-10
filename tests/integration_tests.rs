use serde::{Serialize, Deserialize};
use tokio;
use rand;
use reqwest;

#[derive(Serialize)]
struct User {
    name: Option<String>,
    username: String,
    email: String,
    password_hash: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub msg: String,
}


mod tests {
    use rand::Rng;

    use super::*;
    use reqwest::StatusCode;


    #[tokio::test]
    async fn test_post_request() {

        let mut rng = rand::thread_rng();

        let user = User {
            name: Some(format!("String {}", rng.gen::<u16>())),
            username: format!("String {}", rng.gen::<u16>()),
            email: format!("String {}", rng.gen::<u16>()),
            password_hash: format!("String {}", rng.gen::<u16>()),
        };
        
        let url_request = "http://127.0.0.1:8000/users/";
        let client = reqwest::Client::new();

        let response = client.post(url_request)
            .json(&user)
            .send()
            .await
            .expect("Failed to send request");
        

        assert_eq!(response.status(), StatusCode::CREATED);


    }


    #[tokio::test]
    async fn test_cannot_post_same() {
        let mut rng = rand::thread_rng();

        let user = User {
            name: Some(format!("String {}", rng.gen::<u16>())),
            username: format!("String {}", rng.gen::<u16>()),
            email: format!("String {}", rng.gen::<u16>()),
            password_hash: format!("String {}", rng.gen::<u16>()),
        };
    
        let url_request = "http://127.0.0.1:8000/users/";
        let client = reqwest::Client::new();

        let _response1 = client.post(url_request)
            .json(&user)
            .send()
            .await
            .expect("Failed to send POST request");

        let response2 = client.post(url_request)
            .json(&user)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_ne!(response2.status(), StatusCode::OK);
    }


    #[tokio::test]
    async fn test_post_and_get() {
        let mut rng = rand::thread_rng();

        let user = User {
            name: Some(format!("String {}", rng.gen::<u16>())),
            username: format!("String {}", rng.gen::<u16>()),
            email: format!("String {}", rng.gen::<u16>()),
            password_hash: format!("String {}", rng.gen::<u16>()),
        };
    
        let url_request = "http://127.0.0.1:8000/users/";
        let client = reqwest::Client::new();

        let response = client.post(url_request)
            .json(&user)
            .send()
            .await
            .expect("Failed to send POST request");

        let create_response: CreateUserResponse = response.json().await.expect("Failed to parse response");
        let user_id: String = create_response.id;

        let get_url = format!("http://127.0.0.1:8000/users/{user_id}");
        let get_response = client.get(&get_url)
            .send()
            .await
            .expect("Failed to send GET request");

        assert_eq!(get_response.status(), StatusCode::OK);
    }


    #[tokio::test]
    async fn test_post_request_missing_field() {
        let user = User {
            name: None,
            username: "String6".to_string(),
            email: "String6".to_string(),
            password_hash: "String6".to_string(),
        };

        let url_request = "http://127.0.0.1:8000/users/";
        let client = reqwest::Client::new();

        let response = client.post(url_request)
            .json(&user)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

}