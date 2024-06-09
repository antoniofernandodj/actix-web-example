use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PostExampleRequest {
    pub data: String
}


#[derive(Deserialize, Serialize)]
pub struct PostExampleResponse {
    data: String,
    username: String,
    id: String
}

impl PostExampleResponse {
    pub fn new(data: String, username: String, id: String) -> Self {
        PostExampleResponse{data, username, id}
    }
}
