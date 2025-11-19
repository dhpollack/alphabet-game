use crate::components::language_selector::LanguageSelector;
use crate::game::GameContext;
use crate::speech::speak;
use leptos::prelude::*;

#[component]
pub fn GameHeader() -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let state = game_context.state;
    let game_context_backspace = game_context.clone();
    let game_context_check = game_context.clone();

    view! {
        <header class="bg-teal-700 text-white p-4 flex items-center justify-between">
            // Left Section: Score, Help, Language
            <div class="flex items-start space-x-4">
                <div class="flex space-x-2">
                    <img src="/icons/star.svg" alt="Score" class="object-fill" />
                    <span class="text-[2vw]">{move || state.get().score}</span>
                </div>
            </div>

            // Center Section: Current Word and User Input
            <div class="flex flex-col items-center">
                <div class="flex items-center space-x-3">
                    <button
                        on:click=move |_| { game_context_backspace.remove_last_letter(); }
                        class="bg-red-500 text-white p-1 rounded hover:bg-red-600 active:bg-red-700 transition-colors"
                    >
                        <img src="/icons/backspace.svg" alt="Backspace" class="w-6 h-6" />
                    </button>
                    <button
                        on:click=move |_| {
                            let word = game_context.get_current_word();
                            if !word.is_empty() {
                                let lang = game_context.lang_code.get();
                                speak(&word, &lang);
                            }
                        }
                        class="bg-transparent border-none p-0 m-0 text-xl font-bold underline cursor-pointer"
                    >
                        {move || state.get().current_word}
                    </button>
                    <button
                        on:click=move |_| {
                            let word = state.get().user_input;
                            if !word.is_empty() {
                                let lang = game_context_check.lang_code.get();
                                speak(&word, &lang);
                            }
                            game_context_check.check_spelling();
                        }
                        class="bg-green-500 text-white p-1 rounded hover:bg-green-600 active:bg-green-700 transition-colors"
                    >
                        <img src="/icons/check.svg" alt="Check" class="w-6 h-6" />
                    </button>
                </div>
                <div class="text-lg mt-1 min-h-6">
                    {move || state.get().user_input}
                </div>
            </div>

            // Right Section: Menu, Control Buttons, and Alphabet
            <div class="flex items-start space-x-4">
                <div class="flex flex-col items-end space-y-2">
                    <div class="flex space-x-2">
                        <button class="text-xl hover:bg-teal-600 p-2 rounded">
                            <img src="/icons/menu.svg" alt="Menu" class="w-6 h-6" />
                        </button>
                        <button class="text-xl hover:bg-teal-600 p-2 rounded">
                            <img src="/icons/alphabet.svg" alt="Alphabet" class="w-6 h-6" />
                        </button>
                    </div>
                    <div class="flex space-x-2">
                        <button class="text-xl hover:bg-teal-600 p-2 rounded">
                            <img src="/icons/help.svg" alt="Help" class="w-6 h-6" />
                        </button>
                        <LanguageSelector />
                    </div>
                </div>
            </div>
        </header>
    }
}
