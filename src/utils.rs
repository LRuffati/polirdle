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

pub enum GuessRet {
    Guessed {ret: Vec<u8>, compat: SizeList, state: GameOn},
    NotPresent (GameOn),
    GameFail (GameOver),
}