use std::collections::VecDeque;

/// `InputEvent` a queue.
pub type InputEvents = VecDeque<InputEvent>;
pub type ResultEvents = VecDeque<ResultEvent>;

/// An `InputEvent` is an event which is processed by
/// a `GameElement`.
///
/// The possible values are:
/// - PlayerMove(direction): Move the player towards `direction`.
/// - EnemyRelease { id }: Allow enemy with number `id` to move.
/// - GamePause: Stop the game keeping the state.
/// - GameQuit: Set the system to exit.
pub enum InputEvent {
    PlayerMove(Direction),
    EnemyRelease { id: usize },
    GamePause,
    GameQuit,
}

/// A `ResultEvent`is an event produced by a `GameElement`
/// as a consequence of processing an `InputEvent`.
///
/// The possibe values are:
/// - DoNothing: Discard event.
/// - GamePause: Stop the game keeping the state.
/// - GameExit: Leave the main loop.
/// - PlayerDied: The player died so exit the game.
/// - EnemyDied { id }: Remove enemy with number `id`.
/// - EnemyBlock { id }: Do not allow enemy with `id` to move.
/// - NextLevel: Change the game to the next level.
pub enum ResultEvent {
    DoNothing,
    GamePause,
    GameExit,
    PlayerDied,
    EnemyDied { id: usize },
    EnemyBlock { id: usize },
    NextLevel,
}

/// It defines the four directions that can be take by the
/// player.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
