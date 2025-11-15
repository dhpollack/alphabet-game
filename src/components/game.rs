use leptos::prelude::*;
use crate::game::GameContext;
use crate::database::get_random_word_for_language;
use crate::components::{header::GameHeader, letter_grid::LetterGrid};

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
                match get_random_word_for_language(1).await {
                    Ok(Some(word)) => {
                        game_context.set_current_word(word.word, word.language_id);
                    }
                    Ok(None) => {
                        leptos::logging::log!("No words found in database");
                    }
                    Err(e) => {
                        leptos::logging::log!("Error loading word: {:?}", e);
                    }
                }
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
                            game_context.reset_for_next_word(word.word, word.language_id);
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

    view! {
        <div class="min-h-screen flex flex-col">
            <GameHeader />
            <LetterGrid />
        </div>
    }
}