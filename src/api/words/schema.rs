use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateWordRequest {
    data: String,
    language_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateWordResponse {
    id: String,
    msg: String,
    data: CreateWordRequest
}

impl CreateWordResponse {
    pub fn new(msg: String, id: String, data: CreateWordRequest) -> Self {
        CreateWordResponse{id, msg, data}
    }
}

#[derive(Serialize, Deserialize)]
pub struct Word {
    id: String,
    language_id: String,
    data: String
}

impl Word {
    pub fn new(id: String, language_id: String, data: String) -> Self {
        Word{id, language_id, data}
    }
}

#[derive(Serialize, Deserialize)]
pub struct Words {
    data: Vec<Word>
}

impl Words {
    pub fn new(words: Vec<Word>) -> Self {
        Words{data: words}
    }
}
