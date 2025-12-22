use crate::database::get_languages;
use crate::game::GameContext;
use leptos::{ev::Event, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{HtmlOptionElement, HtmlSelectElement};

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let game_context = use_context::<GameContext>().expect("GameContext should be provided");

    // Fetch available languages
    let languages_resource = OnceResource::new(get_languages());

    // Handle language change
    let on_language_change = move |ev: Event| {
        let target = event_target::<HtmlSelectElement>(&ev);
        let value = target.value();
        if let Ok(_language_id) = value.parse::<u32>() {
            let selected_index = target.selected_index() as u32;
            if let Some(option) = target.item(selected_index)
                && let Ok(option_element) = option.dyn_into::<HtmlOptionElement>()
            {
                let lang_code = option_element.label();
                let new_language = languages_resource
                    .get()
                    .expect("unable to get languages")
                    .expect("server error getting languages")
                    .iter()
                    .find(|&l| l.code == lang_code)
                    .cloned()
                    .expect("selected language not found");
                game_context.current_language.set(new_language);
            }
        }
    };

    view! {
        <div class="flex items-center space-x-2">
            <Suspense fallback=move || view! { <p class="text-white text-center">"Loading..."</p> }>
                <select
                    on:change=on_language_change
                    class="bg-teal-800 text-white px-2 py-1 rounded border border-teal-600 focus:outline-none focus:ring-2 focus:ring-teal-400"
                    prop:value=move || game_context.current_language.get().id
                >
                    {move || {
                        match languages_resource.get() {
                            Some(Ok(languages)) => {
                                languages.into_iter().map(|language| {
                                    view! {
                                        <option value={language.id.to_string()} label={language.code.clone().to_string()}>
                                            {language.code.clone()}
                                        </option>
                                    }
                                }).collect_view().into_any()
                            }
                            Some(Err(_)) => {
                                view! { <option value="1">en</option> }.into_any()
                            }
                            None => {
                                view! { <option value="1">Loading...</option> }.into_any()
                            }
                        }
                    }}
                </select>
            </Suspense>
        </div>
    }
}
