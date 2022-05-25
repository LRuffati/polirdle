use std::num::NonZeroU8;
use crate::states::{GameOn, GameOver};

/// A numerical type which can hold the length of a list of compatible words
pub type SizeList = u16;
pub type WordSize = u8;
pub type InputType = NonZeroU8;

#[derive(Eq, PartialEq)]
pub enum CompFlag {
    Match,
    NotPresent,
    WrongPosition,
    /// Same representation as not present, but the letter in this position was
    /// present in the guess more times than in the target
    WrongPositionOverflow,
}

impl CompFlag {
    pub fn to_symbol(&self) -> char {
        match self {
            CompFlag::Match => {'+'}
            CompFlag::NotPresent | CompFlag::WrongPositionOverflow => {'/'}
            CompFlag::WrongPosition => {'|'}
        }
    }

    pub fn vec_to_res(vec: &Vec<Self>) -> String {
        vec.iter().map(|r| r.to_symbol()).collect()
    }
}

pub enum GuessRet {
    Attempt {ret: Vec<char>, compat: SizeList, state: GameOn},
    NotPresent (GameOn),
    GameFail (GameOver),
    GameWon (GameOver),
}

