use crate::components::letter_button::LetterButton;
use crate::game::GameContext;
use leptos::prelude::*;

#[component]
pub fn LetterGrid() -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let state = game_context.state;

    view! {
        <div class="bg-yellow-200 flex-grow">
            <div class="w-full h-full">
                <Suspense fallback=move || view! { <p class="text-white text-center">"Loading..."</p> }>
                    { move || {
                        let game_letters = state.with(|s| s.game_letters.clone());

                        view! {
                            <div class="grid grid-cols-3 landscape:grid-cols-4 h-full">
                                {game_letters.into_iter().map(|letter| {
                                    view! {
                                        <LetterButton letter=letter />
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }}
                </Suspense>
            </div>
        </div>
    }
}
