use hyper::{Body, Client, Method, Request, Response, StatusCode};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use tokio;
use rand;

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

async fn make_post_request(user: &User) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
    let client: Client<HttpsConnector<HttpConnector>> = Client::builder().build::<_, hyper::Body>(https);

    let json = serde_json::to_string(&user)?;

    let req: Request<Body> = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:8000/users/")
        .header("Content-Type", "application/json")
        .body(Body::from(json))?;

    let resp: Response<Body> = client.request(req).await?;

    Ok(resp)
}

async fn make_get_request(id: &str) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
    let client: Client<HttpsConnector<HttpConnector>> = Client::builder().build::<_, hyper::Body>(https);

    let req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri(format!("http://127.0.0.1:8000/users/{id}"))
        .header("Content-Type", "application/json")
        .body(Body::empty())?;

    let resp: Response<Body> = client.request(req).await?;

    Ok(resp)
}

mod tests {
    use rand::Rng;

    use super::*;

    #[tokio::test]
    async fn test_post_request() {

        let mut rng = rand::thread_rng();

        let user = User {
            name: Some(format!("String {}", rng.gen::<u16>())),
            username: format!("String {}", rng.gen::<u16>()),
            email: format!("String {}", rng.gen::<u16>()),
            password_hash: format!("String {}", rng.gen::<u16>()),
        };
    
        let response = make_post_request(&user).await.expect("Failed to make request");
    
        assert_eq!(response.status(), StatusCode::CREATED);
    
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response body");
        let body_str = String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8");
    
        println!("Response: {}", body_str);
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
    
        let response = make_post_request(&user).await.expect("Failed to make request");
        
        assert_eq!(response.status(), StatusCode::CREATED);
    
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response body");
        let body_str = String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8");
        let response_serialized: Result<CreateUserResponse, serde_json::Error> = serde_json::from_str(&body_str);
        
        assert!(response_serialized.is_ok());
        
        let r = response_serialized.unwrap();
        let resp_result = make_get_request(&r.id).await;
        
        assert!(resp_result.is_ok());
        
        let resp: Response<Body> = resp_result.unwrap();
        
        assert_eq!(resp.status(), 200);


        println!("Response: {}", body_str);
    }


    #[tokio::test]
    async fn test_post_request_missing_field() {
        let user = User {
            name: None,
            username: "String6".to_string(),
            email: "String6".to_string(),
            password_hash: "String6".to_string(),
        };

        let response = make_post_request(&user).await.expect("Failed to make request");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body_bytes = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response body");
        let body_str = String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8");

        println!("Response: {}", body_str);
    }

}