use std::fmt::Display;
use std::num::NonZeroU8;
use crate::states::{GameOn, GameOver};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// A numerical type which can hold the length of a list of compatible words
pub type SizeList = u16;
pub type WordSize = u8;
pub type InputType = NonZeroU8;

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
}

pub enum GuessRet {
    Attempt {ret: Vec<char>, compat: SizeList, state: GameOn},
    NotPresent (GameOn),
    GameFail (GameOver),
    GameWon (GameOver),
}

