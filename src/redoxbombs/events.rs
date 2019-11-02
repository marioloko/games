use std::collections::VecDeque;

/// `InputEvent` a queue.
pub type InputEvents = VecDeque<InputEvent>;
pub type ResultEvents = VecDeque<ResultEvent>;

/// An `InputEvent` is an event which is processed by
/// a `GameElement`. They are usually triggered by an
/// user input.
pub enum InputEvent {
    /// Move the player towards `direction`.
    PlayerMove(Direction),

    /// Allow enemy with number `id` to move.
    /// It is NOT triggered by a user input.
    EnemyRelease { id: usize },

    /// Allow a `Stairs` object to take a turn.
    /// It is NOT triggered by a user input.
    StairsRelease,

    /// Stop the game keeping the state.
    GamePause,

    /// Set the system to exit.
    GameQuit,
}

/// A `ResultEvent`is an event produced by a `GameElement`
/// as a consequence of processing an `InputEvent`.
///
/// The possibe values are:
pub enum ResultEvent {
    /// Discard event. Used when no extra action is required.
    DoNothing,

    /// Stop the game keeping the state.
    GamePause,

    /// Leave the main loop.
    GameExit,

    /// The player died so exit the game.
    PlayerDied,

    /// Remove enemy with number `id`.
    EnemyDied { id: usize },

    /// Do not allow enemy with `id` to move.
    EnemyBlock { id: usize },

    /// Do not allow stairs to take turn.
    StairsBlock,

    /// Change the game to the next level.
    NextLevel,
}

/// It defines the four directions that can be take by the
/// player.
pub enum Direction {
    /// Towards north.
    Up,

    /// Towards south.
    Down,

    /// Toards west.
    Left,

    /// Toards east.
    Right,
}
