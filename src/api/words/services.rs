use actix_web::{Responder, web::Path, web::Json};
use crate::api::words::schema::{CreateWordRequest, CreateWordResponse, Word, Words};
use crate::prelude::*;


pub async fn get_word(path: Path<String>) -> impl Responder {
    let id: String = path.into_inner();
    let language_id = uuid::Uuid::new_v4().to_string();
    let word: Word = Word::new(id, language_id, "Ola".to_owned());
    json_response(word, None)
}

pub async fn get_words() -> impl Responder {
    let language_id: String = uuid::Uuid::new_v4().to_string();

    let id_word1: String = uuid::Uuid::new_v4().to_string();
    let id_word2: String = uuid::Uuid::new_v4().to_string();

    let word1: Word = Word::new(
        id_word1,
        language_id.to_owned(),
        "Ola".to_owned()
    );

    let word2: Word = Word::new(
        id_word2,
        language_id.to_owned(),
        "Mundo".to_owned()
    );

    let data: Vec<Word> = vec![word1, word2];
    let words: Words = Words::new(data);

    json_response(words, None)
}

pub async fn create_word(body: Json<CreateWordRequest>) -> impl Responder {
    let data: CreateWordRequest = body.into_inner();

    let id_word1: String = uuid::Uuid::new_v4().to_string();

    let response: CreateWordResponse = CreateWordResponse::new(
        "Criado com sucesso!".to_owned(), id_word1, data
    );
    json_response(response, Some(201))
}
