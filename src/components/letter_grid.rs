use crate::components::letter_button::LetterButton;
use crate::game::GameContext;
use leptos::prelude::*;

#[component]
pub fn LetterGrid() -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let state = game_context.state;

    view! {
        <div class="bg-amber-800 min-h-screen p-4">
            <div class="max-w-md mx-auto">
                <Suspense fallback=move || view! { <p class="text-white text-center">"Loading..."</p> }>
                    { move || {
                        let game_letters = state.with(|s| s.game_letters.clone());

                        view! {
                            <div class="grid grid-cols-4 gap-4">
                                {game_letters.into_iter().map(|letter| {
                                    let game_context_clone = game_context.clone();
                                    let letter_clone = letter.clone();
                                    view! {
                                        <LetterButton
                                            letter=letter
                                            on_click=move || {
                                                game_context_clone.add_letter(&letter_clone);
                                            }
                                        />
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
