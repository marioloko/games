/// An `InputEvent` is an event produced as a consequence
/// of a user input. 
pub enum InputEvent {
    /// Move the player towards `direction`.
    PlayerMove(Direction),

    /// Stop the game keeping the state.
    GamePause,

    /// Set the system to exit.
    GameQuit,
}

/// A `GameEvent` is an event generated by the game to allow
/// their elements to handle requests in a RoundRobin way.
pub enum GameEvent {
    /// Allow enemy with number `id` to check collision 
    /// with other game elements.
    EnemyCheckCollision { id: usize },

    /// Allow enemy with number `id` to move.
    EnemyRelease { id: usize },

    /// Allow a `Stairs` object to take a turn.
    StairsRelease,
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

    /// Allow enemy to check collision with other game elements.
    EnemyCheckCollision { id: usize },

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
