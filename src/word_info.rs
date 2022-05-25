use std::rc::Rc;
use crate::utils::{InputType, WordSize};

/// A type used to track letters mapped to a number, will be used in the following cases:
/// 1. Handling right letter multiple times when evaluating
/// 2. Tracking the minimum / maximum / exact number of times a given letter appears in the target
#[derive(Clone)]
pub struct TinyMap {
    data: Vec<(InputType, WordSize)>,
}

pub enum TinyRes {
    Ok,
    Overflow,
    NotPresent,
}

impl TinyMap {
    pub fn from_vec(word: &Vec<InputType>) -> Self {
        let mut data = Vec::with_capacity(word.len());
        let mut orig: Vec<InputType> = word.iter().map(|&i| i).collect();
        orig.sort();
        let mut res: (Vec<(InputType, WordSize)>, Option<(InputType, WordSize)>) = orig.iter().fold(
            (data, None), |(mut acc, curr), &b|
                {
                    return if let Some((p, i)) = curr {
                        if p == b {
                            (acc, Some((p, i + 1)))
                        } else {
                            acc.push((p, i));
                            (acc, Some((b, 1)))
                        }
                    } else {
                        (acc, Some((b, 1)))
                    };
                });
        if let Some(x) = res.1 {
            res.0.push(x);
        }
        TinyMap { data: res.0 }
    }

    fn find(&self, c: InputType) -> Result<usize, usize> {
        self.data.binary_search_by(|&a| a.0.cmp(&c))
    }

    /// Decrease a letter counter, return false if it was 0 already
    pub fn pop_letter(&mut self, c: InputType) -> bool {
        let res = self.find(c);
        return match res {
            Ok(idx) => {
                if self.data[idx].1 >= 1 {
                    self.data[idx].1 -= 1;
                    true
                } else {
                    false
                }
            }
            Err(_) => {
                false
            }
        }
    }

    /// Increments a letter counter
    pub fn add_letter(&mut self, c: InputType) {
        match self.find(c) {
            Ok(idx) => {
                self.data[idx].1 += 1;
            }
            Err(idx) => {
                self.data.insert(idx, (c, 1))
            }
        }
    }

    /// Get the value of a letter counter
    pub fn get_letter(&self, c: InputType) -> WordSize {
        match self.find(c) {
            Ok(idx) => {self.data[idx].1}
            Err(_) => {0}
        }
    }

    /// Immediately set a letter to the given value
    pub fn set_letter(&mut self, c: InputType, v: WordSize) {
        match self.find(c) {
            Ok(idx) => {self.data[idx].1 = v}
            Err(idx) => {self.data.insert(idx, (c, v))}
        }
    }

    /// Remove a letter entry and return the value it had
    pub fn clear_letter(&mut self, c: InputType) -> WordSize {
        match self.find(c) {
            Ok(idx) => {self.data.remove(idx).1}
            Err(_) => {0}
        }
    }

    pub fn test_membership(&self, c: InputType) -> bool {
        match self.find(c) {
            Ok(idx) => {self.data[idx].1 > 0}
            Err(_) => {false}
        }
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