use std::collections::HashSet;

use leptos::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::database::Language;

const GAME_GRID_SIZE: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_word: String,
    pub user_input: String,
    pub score: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub current_attempt: i32,
    pub is_completed: bool,
    pub language: Language,
    pub language_letters: Vec<String>,
    pub game_letters: Vec<String>,
    pub game_grid_size: usize,
}

impl GameState {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            current_word: String::new(),
            user_input: String::new(),
            score: 0,
            attempts: 0,
            max_attempts: 5,
            current_attempt: 1,
            is_completed: false,
            language_letters: vec![],
            game_letters: vec![],
            game_grid_size: GAME_GRID_SIZE,
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

    pub fn reset_for_next_word(&mut self, next_word: String) {
        let alphabet_letters = self.language_letters.clone();
        let grid_size = self.game_grid_size;
        let mut grid_letters: HashSet<String> = next_word.chars().map(|c| c.to_string()).collect();
        let mut rng = rand::rng();
        let mut distractor_letters: Vec<String> = alphabet_letters
            .into_iter()
            .filter(|l| !grid_letters.contains(l))
            .collect();
        distractor_letters.shuffle(&mut rng);
        let needed = grid_size.saturating_sub(grid_letters.len());
        grid_letters.extend(distractor_letters.into_iter().take(needed));
        let mut final_grid: Vec<String> = grid_letters.into_iter().collect();
        final_grid.shuffle(&mut rng);

        self.current_word = next_word;
        self.user_input.clear();
        self.attempts = 0;
        self.current_attempt = 1;
        self.is_completed = false;
        self.game_letters = final_grid;
    }

    pub fn set_language_letters(&mut self, letters: Vec<String>) {
        self.language_letters = letters;
    }

    pub fn set_game_letters(&mut self, letters: Vec<String>) {
        self.game_letters = letters;
    }
}

#[derive(Debug, Clone)]
pub struct GameContext {
    pub state: RwSignal<GameState>,
    pub current_language: RwSignal<Language>,
}

impl GameContext {
    pub fn new(language: Language) -> Self {
        Self {
            state: RwSignal::new(GameState::new(language.clone())),
            current_language: RwSignal::new(language),
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

    pub fn set_language(&self, language: &Language) {
        self.state.update(|state| {
            state.language = language.clone();
        });
        self.current_language.set(language.clone());
    }

    pub fn reset_for_next_word(&self, next_word: String) {
        self.state.update(|state| {
            state.reset_for_next_word(next_word);
        });
    }

    pub fn set_language_letters(&self, letters: Vec<String>) {
        self.state.update(|state| {
            state.set_language_letters(letters);
        });
    }

    pub fn get_current_word(&self) -> String {
        self.state.get().current_word.clone()
    }

    pub fn get_language(&self) -> Language {
        self.state.get().language
    }
}
