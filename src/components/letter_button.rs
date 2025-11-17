use leptos::prelude::*;

use crate::game::GameContext;

#[component]
pub fn LetterButton(letter: String) -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let letter_clone = letter.clone();
    let synth = window().speech_synthesis();
    let utterance = web_sys::SpeechSynthesisUtterance::new_with_text(&letter);
    view! {
        <button
            on:click=move |_| {
                game_context.add_letter(&letter_clone);
                if let Ok(synth) = synth.clone() && let Ok(utterance) = utterance.clone() {
                    synth.speak(&utterance);
                }
            }
            class="bg-transparent text-black text-[5vw] md:text-6xl font-bold w-full h-full hover:bg-black/10 active:bg-gray-200 transition-colors flex items-center justify-center"
        >
            {letter}
        </button>
    }
}
