use leptos::prelude::*;
use crate::game::GameContext;

#[component]
pub fn GameHeader() -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let state = game_context.state;
    let game_context_clone = game_context.clone();

    view! {
        <header class="bg-teal-700 text-white p-4 flex items-center justify-between">
            // Left Section: Score, Help, Language
            <div class="flex items-center space-x-4">
                <div class="flex items-center space-x-2">
                    <span class="text-yellow-400">&starf;</span>
                    <span>{move || state.get().score}</span>
                </div>
                <button class="text-xl hover:bg-teal-600 p-2 rounded">
                    ?
                </button>
                <div class="flex items-center space-x-2">
                    <span class="text-xl">US</span>
                </div>
            </div>

            // Center Section: Current Word and User Input
            <div class="flex flex-col items-center">
                <div class="text-xl font-bold underline">
                    {move || state.get().current_word}
                </div>
                <div class="text-lg mt-1 min-h-6">
                    {move || state.get().user_input}
                </div>
            </div>

            // Right Section: Menu, Control Buttons, and Alphabet
            <div class="flex items-center space-x-4">
                <div class="flex flex-col items-end space-y-2">
                    <div class="flex space-x-2">
                        <button
                            on:click=move |_| { game_context.remove_last_letter(); }
                            class="bg-red-500 text-white px-3 py-1 rounded text-sm hover:bg-red-600 active:bg-red-700 transition-colors"
                        >
                            &larr;
                        </button>
                        <button
                            on:click=move |_| { game_context_clone.check_spelling(); }
                            class="bg-green-500 text-white px-3 py-1 rounded text-sm hover:bg-green-600 active:bg-green-700 transition-colors"
                        >
                            &check;
                        </button>
                    </div>
                    <div class="flex space-x-2">
                        <button class="text-xl hover:bg-teal-600 p-2 rounded">
                            &#9776;
                        </button>
                        <button class="text-xl hover:bg-teal-600 p-2 rounded">
                            <span class="underline">A</span>
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}