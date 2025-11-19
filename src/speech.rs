use leptos::prelude::*;

pub fn speak(text: &str, lang: &str) {
    if text.is_empty() {
        return;
    }
    if let Ok(synth) = window().speech_synthesis() {
        match web_sys::SpeechSynthesisUtterance::new_with_text(text) {
            Ok(utterance) => {
                utterance.set_lang(lang);
                synth.speak(&utterance);
            }
            Err(e) => {
                let error_message = format!("Error creating utterance: {:?}", e);
                leptos::logging::error!("{}", error_message);
            }
        }
    }
}
