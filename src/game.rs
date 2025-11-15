use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_word: String,
    pub user_input: String,
    pub score: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub current_attempt: i32,
    pub is_completed: bool,
    pub language_id: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_word: String::new(),
            user_input: String::new(),
            score: 0,
            attempts: 0,
            max_attempts: 5,
            current_attempt: 1,
            is_completed: false,
            language_id: 1, // Default language ID
        }
    }
}

impl GameState {
    pub fn new(word: String, language_id: u32) -> Self {
        Self {
            current_word: word,
            language_id,
            ..Default::default()
        }
    }

    pub fn add_letter(&mut self, letter: &str) {
        if self.user_input.chars().count() < self.current_word.chars().count() {
            self.user_input.push_str(letter);
        }
    }

    pub fn remove_last_letter(&mut self) {
        if let Some(last_char_boundary) = self.user_input.char_indices().last() {
            self.user_input.truncate(last_char_boundary.0);
        }
    }

    pub fn check_spelling(&mut self) -> bool {
        self.attempts += 1;
        let is_correct = self.user_input == self.current_word;

        if is_correct {
            // Calculate points: 1 point per character + bonus for first try
            let base_points = self.current_word.chars().count() as i32;
            let bonus_points = if self.attempts == 1 {
                10
            } else {
                (10 - (self.attempts - 1) * 2).max(0)
            };
            self.score += base_points + bonus_points;
            self.is_completed = true;
        } else if self.attempts >= self.max_attempts {
            self.is_completed = true;
        }

        is_correct
    }

    pub fn reset_for_next_word(&mut self, next_word: String, language_id: u32) {
        self.current_word = next_word;
        self.user_input.clear();
        self.attempts = 0;
        self.current_attempt = 1;
        self.is_completed = false;
        self.language_id = language_id;
    }
}

#[derive(Debug, Clone)]
pub struct GameContext {
    pub state: RwSignal<GameState>,
    pub current_language: RwSignal<u32>,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(GameState::default()),
            current_language: RwSignal::new(1), // Default language ID
        }
    }

    pub fn add_letter(&self, letter: &str) {
        self.state.update(|state| {
            state.add_letter(letter);
        });
    }

    pub fn remove_last_letter(&self) {
        self.state.update(|state| {
            state.remove_last_letter();
        });
    }

    pub fn check_spelling(&self) -> bool {
        let mut result = false;
        self.state.update(|state| {
            result = state.check_spelling();
        });
        result
    }

    pub fn set_current_word(&self, word: String, language_id: u32) {
        self.state.update(|state| {
            state.current_word = word;
            state.language_id = language_id;
            state.user_input.clear();
            state.attempts = 0;
            state.current_attempt = 1;
            state.is_completed = false;
        });
        self.current_language.set(language_id);
    }

    pub fn reset_for_next_word(&self, next_word: String, language_id: u32) {
        self.state.update(|state| {
            state.reset_for_next_word(next_word, language_id);
        });
        self.current_language.set(language_id);
    }
}