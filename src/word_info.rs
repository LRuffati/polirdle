use std::rc::Rc;
use crate::utils::{InputType, WordSize};

/// A type used to track letters mapped to a number, will be used in the following cases:
/// 1. Handling right letter multiple times when evaluating
/// 2. Tracking the minimum / maximum / exact number of times a given letter appears in the target
#[derive(Copy, Clone)]
pub struct TinyMap {}

impl TinyMap {
    pub fn from_vec(word: &Vec<InputType>) -> Self {
        todo!()
    }

    /// Decrease a letter counter, return false if it was 0 already
    pub fn pop_letter(self, c: InputType) -> bool {
        todo!()
    }

    /// Increments a letter counter
    pub fn add_letter(self, c: InputType) {
        todo!()
    }

    /// Get the value of a letter counter
    pub fn get_letter(self, c: InputType) -> WordSize {
        todo!()
    }

    /// Immediately set a letter to the given value
    pub fn set_letter(self, c: InputType, v: WordSize) {
        todo!()
    }

    /// Remove a letter entry and return the value it had
    pub fn clear_letter(self, c: InputType) -> WordSize {
        todo!()
    }

    pub fn test_membership(self, c: InputType) -> bool {
        todo!()
    }
}

/// The information of a word
/// Should contain which letters are present, which are repeated (and how much)
/// and the word itself
pub struct WordInfo {}

impl WordInfo {

    /// Compile the info for this word
    pub fn from_word(word: Vec<InputType>) -> Self {
        todo!()
    }

    pub fn get_word(&self) -> Rc<Vec<InputType>> {
        todo!()
    }
}