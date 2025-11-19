use leptos::prelude::*;

use crate::components::{header::GameHeader, letter_grid::LetterGrid};
use crate::database::{get_letters_for_language, get_random_word_for_language};
use crate::game::GameContext;

#[component]
pub fn AlphabetGame() -> impl IntoView {
    let game_context = GameContext::new();
    provide_context(game_context.clone());

    // Load initial word when component mounts
    Effect::new({
        let game_context = game_context.clone();
        move || {
            let game_context = game_context.clone();
            leptos::task::spawn_local(async move {
                let current_language = game_context.get_language_id();
                match get_letters_for_language(current_language).await {
                    Ok(letters_res) if !letters_res.is_empty() => {
                        let alphabet_letters: Vec<String> =
                            letters_res.into_iter().map(|l| l.letter).collect();
                        // set alphabet letters
                        game_context.set_language_letters(alphabet_letters.clone());
                    }
                    Ok(_) => leptos::logging::log!(
                        "Returned empty vec of letters for {current_language}"
                    ),
                    Err(e) => {
                        leptos::logging::log!("No letters found for this language: {:?}", e);
                    }
                };
                match get_random_word_for_language(current_language).await {
                    Ok(Some(word)) => {
                        game_context.reset_for_next_word(word.word);
                    }
                    Ok(None) => {
                        leptos::logging::log!("No words found in database");
                    }
                    Err(e) => {
                        leptos::logging::log!("Error loading word: {:?}", e);
                    }
                };
            });
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
                    match get_random_word_for_language(state.language_id).await {
                        Ok(Some(word)) => {
                            game_context.reset_for_next_word(word.word);
                        }
                        Ok(None) => {
                            leptos::logging::log!("No more words found");
                        }
                        Err(e) => {
                            leptos::logging::log!("Error loading next word: {:?}", e);
                        }
                    }
                });
            }
        }
    });

    // Handle language change
    Effect::new({
        let game_context = game_context.clone();
        move || {
            let state = game_context.state.get();
            let current_language = game_context.current_language.get();
            if state.language_id != current_language {
                let game_context = game_context.clone();
                leptos::task::spawn_local(async move {
                    // set language_id
                    game_context.set_language_id(current_language);
                    match get_letters_for_language(current_language).await {
                        Ok(letters_res) if !letters_res.is_empty() => {
                            let alphabet_letters: Vec<String> =
                                letters_res.into_iter().map(|l| l.letter).collect();
                            // set alphabet letters
                            game_context.set_language_letters(alphabet_letters.clone());
                        }
                        Ok(_) => leptos::logging::log!(
                            "Returned empty vec of letters for {current_language}"
                        ),
                        Err(e) => {
                            leptos::logging::log!("No letters found for this language: {:?}", e);
                        }
                    };
                    match get_random_word_for_language(current_language).await {
                        Ok(Some(word)) => {
                            game_context.reset_for_next_word(word.word);
                        }
                        Ok(None) => {
                            leptos::logging::log!("No words found in database");
                        }
                        Err(e) => {
                            leptos::logging::log!("Error loading word: {:?}", e);
                        }
                    };
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

