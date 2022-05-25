use crate::utils::{GuessRet, InputType, WordSize};

/**
In this file I will define the states of the application and the exposed functionalities
they hold
*/

/// The only structure directly created, its only method will be to accept a maximum
/// length for words in the application
pub struct Init {
}

impl Init {
    pub fn set_length(self, len: WordSize) -> WordCollectorInit {
        todo!()
    }

}

/// At this stage we'll be collecting the initial set of strings
struct WordCollectorInit {
    length: u8,
}

impl WordCollectorInit {
    pub fn add_word(&mut self, word: Vec<InputType>) {
        todo!()
    }

    pub fn new_match(self) -> GameSetWord {
        todo!()
    }
}

/// This state represents a game setup phase waiting for the reference word
struct GameSetWord {}

impl GameSetWord {
    pub fn set_target(self, target: Vec<InputType>) -> GameSetTries {
        todo!()
    }
}

/// Setup phase of a game while waiting for the tries
struct GameSetTries {}

impl GameSetTries {
    pub fn set_tries(self, tries: u8) -> GameOn {
        todo!()
    }
}

/// Active game
pub struct GameOn {}

impl GameOn {
    pub fn guess(self, guess: Vec<InputType>) -> GuessRet {
        todo!()
    }

    /// Start inserting words in the list while playing a game
    pub fn start_insert(self) -> InsertDuringGame {
        todo!()
    }
}

struct InsertDuringGame {}

impl InsertDuringGame {
    pub fn insert(&mut self, word: Vec<InputType>) {
        todo!()
    }

    pub fn return_to_game(self) -> GameOn {
        todo!()
    }
}

/// Outside of an active game
pub struct GameOver {}

impl GameOver {
    pub fn close(self) {
        todo!()
    }

    pub fn new_game(self) -> GameSetWord {
        todo!()
    }

    pub fn start_insert(self) -> InsertOutsideGame {
        todo!()
    }
}

/// In this state we will be inserting words while not during an active game
struct InsertOutsideGame {}

impl InsertOutsideGame {
    pub fn insert(&mut self, word: Vec<InputType>){
        todo!()
    }

    pub fn stop_inserting(self) -> GameOver {
        todo!()
    }
}
