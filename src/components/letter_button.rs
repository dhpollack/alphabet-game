use leptos::prelude::*;

#[component]
pub fn LetterButton(letter: String, on_click: impl Fn() + 'static) -> impl IntoView {
    view! {
        <button
            on:click=move |_| on_click()
            class="bg-white text-black text-2xl font-bold h-16 w-16 rounded-lg shadow-md hover:bg-gray-100 active:bg-gray-200 transition-colors flex items-center justify-center"
        >
            {letter}
        </button>
    }
}
