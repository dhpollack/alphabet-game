use leptos::prelude::*;
use leptos_router::{hooks::use_query, params::Params};
use serde::{Deserialize, Serialize};

use crate::components::{header::GameHeader, letter_grid::LetterGrid};
use crate::database::{
    Language, get_default_language, get_languages, get_letters_for_language,
    get_random_word_for_language,
};
use crate::game::GameContext;

#[derive(Params, PartialEq, Clone, Serialize, Deserialize)]
struct QueryParams {
    lang: Option<String>,
}

#[component]
pub fn AlphabetGame() -> impl IntoView {
    let default_language_resource = OnceResource::new(get_default_language());
    let languages_resource = OnceResource::new(get_languages());
    let query = use_query::<QueryParams>();
    view! {
        <Suspense fallback=|| {
            view! { "Loading..." }
        }>
            {move || {
                let start_language = match (
                    query.get(),
                    languages_resource.get(),
                    default_language_resource.get(),
                ) {
                    (Ok(query_params), _, Some(Ok(default_lang))) if query_params.lang.is_none() => {
                        Some(default_lang)
                    }
                    (Ok(query_params), Some(Ok(languages)), Some(Ok(default_lang))) => {
                        let found_lang = query_params
                            .lang
                            .and_then(|lang_code| {
                                languages.iter().find(|l| l.code == *lang_code).cloned()
                            })
                            .unwrap_or(default_lang);
                        Some(found_lang)
                    }
                    _ => None,
                };
                if let Some(lang) = start_language {
                    view! { <GameContent default_language=lang /> }.into_any()
                } else {
                    view! { "Error!" }.into_any()
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn GameContent(default_language: Language) -> impl IntoView {
    let game_context = GameContext::new(default_language);
    provide_context(game_context.clone());

    async fn next_word(ctx: &GameContext, lang: Language) {
        match get_random_word_for_language(lang.clone()).await {
            Ok(word) => {
                ctx.reset_for_next_word(word);
            }
            Err(e) => {
                leptos::logging::log!("Error loading word: {:?}", e);
            }
        };
    }

    // Load initial word when component mounts
    Effect::new({
        let game_context = game_context.clone();
        move || {
            let game_context = game_context.clone();
            leptos::task::spawn_local(async move {
                let current_language = game_context.get_language();
                match get_letters_for_language(current_language.clone()).await {
                    Ok(letters_res) if !letters_res.is_empty() => {
                        let alphabet_letters: Vec<String> =
                            letters_res.into_iter().map(|l| l.letter).collect();
                        // set alphabet letters
                        game_context.set_language_letters(alphabet_letters.clone());
                    }
                    Ok(_) => leptos::logging::log!(
                        "Returned empty vec of letters for {}",
                        current_language.code
                    ),
                    Err(e) => {
                        leptos::logging::log!("No letters found for this language: {:?}", e);
                    }
                };
                next_word(&game_context, current_language).await;
            });
        }
    });

    // Handle language change
    Effect::new({
        let game_context = game_context.clone();
        move || {
            let state = game_context.state.get();
            let current_language = game_context.current_language.get();
            if state.language.id != current_language.id {
                let game_context = game_context.clone();
                leptos::task::spawn_local(async move {
                    // set language_id
                    game_context.set_language(&current_language.clone());
                    match get_letters_for_language(current_language.clone()).await {
                        Ok(letters_res) if !letters_res.is_empty() => {
                            let alphabet_letters: Vec<String> =
                                letters_res.into_iter().map(|l| l.letter).collect();
                            // set alphabet letters
                            game_context.set_language_letters(alphabet_letters.clone());
                        }
                        Ok(_) => leptos::logging::log!(
                            "Returned empty vec of letters for {}",
                            current_language.code
                        ),
                        Err(e) => {
                            leptos::logging::log!("No letters found for this language: {:?}", e);
                        }
                    };
                    next_word(&game_context, current_language).await;
                });
            }
        }
    });

    // Handle word progression when current word is completed
    Effect::new({
        let game_context = game_context.clone();
        move || {
            let state = game_context.state.get();
            if state.is_completed {
                let game_context = game_context.clone();
                // Wait a moment, then load next word
                leptos::task::spawn_local(async move {
                    next_word(&game_context, state.language).await;
                });
            }
        }
    });

    view! {
        <div class="h-full flex flex-col">
            <GameHeader />
            <LetterGrid />
        </div>
    }
}
