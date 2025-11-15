use crate::components::letter_button::LetterButton;
use crate::database::get_letters_for_language;
use crate::game::GameContext;
use leptos::prelude::*;
use rand::prelude::IndexedRandom;

#[component]
pub fn LetterGrid() -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");
    let state = game_context.state;
    let current_language = game_context.current_language;

    let grid_letters = RwSignal::new(Vec::<String>::new());

    // Load letters when language or word changes
    Effect::new(move || {
        let lang_id = current_language.get();
        let current_word = state.get().current_word.clone();

        leptos::task::spawn_local(async move {
            match get_letters_for_language(lang_id).await {
                Ok(letters_result) => {
                    // Get all available letters for this language
                    let available_letters: Vec<String> = letters_result
                        .into_iter()
                        .map(|letter| letter.letter)
                        .collect();

                    // Get unique letters from current word
                    let mut word_letters: Vec<String> =
                        current_word.chars().map(|c| c.to_string()).collect();

                    // Add random distractor letters to fill a 3x4 grid (12 letters)
                    let needed_letters = 12usize.saturating_sub(word_letters.len());

                    // Get letters that are not in the current word
                    let distractors: Vec<String> = available_letters
                        .into_iter()
                        .filter(|letter| !word_letters.contains(letter))
                        .collect();

                    // Add random distractors
                    use rand::seq::SliceRandom;
                    let mut rng = rand::rng();

                    for _ in 0..needed_letters {
                        if let Some(letter) = distractors.choose(&mut rng) {
                            word_letters.push(letter.clone());
                        }
                    }

                    // Shuffle all letters
                    word_letters.shuffle(&mut rng);

                    grid_letters.set(word_letters);
                }
                Err(e) => {
                    leptos::logging::log!("Error loading letters: {:?}", e);
                }
            }
        });
    });

    view! {
        <div class="bg-amber-800 min-h-screen p-4">
            <div class="max-w-md mx-auto">
                // Letter Grid
                {move || {
                    let letters = grid_letters.get();
                    if letters.is_empty() {
                        view! {
                            <div class="grid grid-cols-4 gap-4">
                                <div class="col-span-4 text-center text-white">Loading letters...</div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="grid grid-cols-4 gap-4">
                                {letters.into_iter().map(|letter| {
                                    let game_context = game_context.clone();
                                    let letter_clone = letter.clone();
                                    view! {
                                        <LetterButton
                                            letter=letter
                                            on_click=move || {
                                                game_context.add_letter(&letter_clone);
                                            }
                                        />
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

