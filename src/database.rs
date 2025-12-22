use std::collections::HashSet;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
const DEFAULT_LANGUAGE_ID: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx_d1::FromRow)]
pub struct Language {
    pub id: u32,
    pub name: String,
    pub name_other: Option<String>,
    pub code: String,
    pub strip_diacritics: bool,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize, sqlx_d1::FromRow)]
pub struct Word {
    pub id: u32,
    pub word: String,
    pub language_id: u32,
}

impl Word {
    pub fn new() -> Self {
        Self::default()
    }

    // Create single word and remove diacritics after loading from database
    pub fn post_process(&self, lang: &Language) -> String {
        let word = self
            .word
            .as_str()
            .split_whitespace()
            .next()
            .map(|s| s.to_string())
            .expect("word should always have non-empty characters");
        match (lang.code.as_str(), lang.strip_diacritics) {
            ("ar", true) => tashkil::remove(&word).to_string(),
            _ => word,
        }
    }

    // Create HashSet of letters to seed the grid before adding distractor letters
    pub fn letters_for_grid(&self) -> HashSet<String> {
        let letters = decompose(self.word.clone());
        letters.into_iter().map(|c| c.to_string()).collect()
    }

    pub fn len(&self) -> usize {
        let letters = decompose(self.word.clone());
        letters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn decompose(word: String) -> Vec<char> {
    match word.chars().next() {
        Some(c) if rustkorean::check_korean(c) => hangeul::decompose(&word)
            .into_iter()
            .flat_map(|block| match block {
                Ok((first, second, Some(third))) => vec![first, second, third],
                Ok((first, second, None)) => vec![first, second],
                _ => vec![],
            })
            .collect(),
        _ => word.chars().collect(),
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

    let languages = sqlx_d1::query_as!(
        Language,
        "SELECT id, name, name_other, code, strip_diacritics FROM Languages"
    )
    .fetch_all(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(languages)
}

#[server]
pub async fn get_letters_for_language(language: Language) -> Result<Vec<Letter>, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let letters = sqlx_d1::query_as!(Letter,
        "SELECT id, letter, language_id, regular, hidden, name_en FROM Letters WHERE language_id = ?",
        language.id
    )
    .fetch_all(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(letters)
}

#[server]
pub async fn get_words_for_language(language: Language) -> Result<Vec<Word>, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let words = sqlx_d1::query_as!(
        Word,
        "SELECT id, word, language_id FROM Words WHERE language_id = ?",
        language.id
    )
    .fetch_all(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(words)
}

#[server]
pub async fn get_random_word_for_language(language: Language) -> Result<Word, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    let word = sqlx_d1::query_as!(
        Word,
        "SELECT id, word, language_id FROM Words WHERE language_id = ? ORDER BY RANDOM()",
        language.id
    )
    .fetch_one(&conn)
    .await
    .map(|mut word| {
        word.word = word.post_process(&language);
        word
    })
    .map_err(|e| worker::Error::RustError(e.to_string()))?;

    Ok(word)
}

#[server]
pub async fn get_default_language() -> Result<Language, ServerFnError> {
    use axum::Extension;
    use std::sync::Arc;
    use worker::Env;

    let Extension::<Arc<Env>>(env) = leptos_axum::extract().await?;
    let d1 = env.d1("alphabet_game_stg")?;
    let conn = sqlx_d1::D1Connection::new(d1);

    match sqlx_d1::query_as!(
        Language,
        "SELECT id, name, name_other, code, strip_diacritics FROM Languages WHERE id = ?",
        DEFAULT_LANGUAGE_ID
    )
    .fetch_one(&conn)
    .await
    .map_err(|e| worker::Error::RustError(e.to_string()))
    {
        Err(err) => {
            leptos::logging::error!("default language error: {err}");
            let server_error = ServerFnError::ServerError(err.to_string());
            Err(server_error)
        }
        Ok(default_langauage) => Ok(default_langauage),
    }
}
