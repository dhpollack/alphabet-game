use leptos::prelude::*;

#[component]
pub fn LetterButton(letter: String, on_click: impl Fn() + 'static) -> impl IntoView {
    view! {
        <button
            on:click=move |_| on_click()
            class="bg-transparent text-black text-[5vw] md:text-6xl font-bold w-full h-full hover:bg-black/10 active:bg-gray-200 transition-colors flex items-center justify-center"
        >
            {letter}
        </button>
    }
}
