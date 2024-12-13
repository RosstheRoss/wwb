use bincode::{Decode, Encode};

/// WWB is a 2-player game at heart. Hardcode 2 players only so no one else has to suffer
pub const PLAYER_COUNT: usize = 2;
pub const BOARD_SIZE: u16 = 100;

#[derive(Encode, Decode, Debug, Default, PartialEq)]
pub struct Game {
    /// We're going to need an insanely large number for the turn count. Previous attempts crashed at trillions of turns.
    pub turn_count: u128,
    /// The players of this "game".
    pub players: [Player; PLAYER_COUNT],
}

#[derive(Encode, Decode, Debug, Default, PartialEq)]
pub struct Player {
    // name: String,
    /// The current space the player is on. The default game board is a 1D array of 100 spaces.
    pub current_space: u16,
    /// The best space they have ever been on.
    pub high_score: u16,
}
