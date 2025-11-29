use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx_d1::FromRow)]
pub struct Language {
    pub id: u32,
    pub name: String,
    pub name_other: Option<String>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx_d1::FromRow)]
pub struct Letter {
    pub id: u32,
    pub letter: String,
    pub language_id: u32,
    pub regular: Option<bool>,
    pub hidden: Option<bool>,
    pub name_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx_d1::FromRow)]
pub struct Word {
    pub id: u32,
    pub word: String,
    pub language_id: u32,
}

impl Word {
    pub fn first_word_no_spaces(&self) -> String {
        self.word
            .as_str()
            .split_whitespace()
            .next()
            .map(|s| s.to_string())
            .expect("word should always have non-empty characters")
    }
}

#[server]
pub async fn get_languages() -> Result<Vec<Language>, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let languages =
        sqlx_d1::query_as!(Language, "SELECT id, name, name_other, code FROM Languages")
            .fetch_all(&conn)
            .await
            .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(languages)
}

#[server]
pub async fn get_letters_for_language(language_id: u32) -> Result<Vec<Letter>, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let letters = sqlx_d1::query_as!(Letter,
        "SELECT id, letter, language_id, regular, hidden, name_en FROM Letters WHERE language_id = ?",
        language_id
    )
    .fetch_all(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(letters)
}

#[server]
pub async fn get_words_for_language(language_id: u32) -> Result<Vec<Word>, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let words = sqlx_d1::query_as!(
        Word,
        "SELECT id, word, language_id FROM Words WHERE language_id = ?",
        language_id
    )
    .fetch_all(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(words)
}

#[server]
pub async fn get_random_word_for_language(language_id: u32) -> Result<Option<Word>, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let word = sqlx_d1::query_as!(
        Word,
        "SELECT id, word, language_id FROM Words WHERE language_id = ? ORDER BY RANDOM() LIMIT 1",
        language_id
    )
    .fetch_optional(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(word)
}
