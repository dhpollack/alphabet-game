use crate::game::GameContext;
use crate::speech::speak;
use leptos::prelude::*;

#[component]
pub fn LetterButton(letter: String) -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let letter_clone = letter.clone();

    view! {
        <button
            on:click=move |_| {
                game_context.add_letter(&letter_clone);
                let lang = game_context.lang_code.get();
                speak(&letter_clone, &lang);
            }
            class="bg-transparent text-black text-[5vw] md:text-6xl font-bold w-full h-full hover:bg-black/10 active:bg-gray-200 transition-colors flex items-center justify-center"
        >
            {letter}
        </button>
    }
}
