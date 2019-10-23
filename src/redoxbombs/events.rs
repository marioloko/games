use std::collections::VecDeque;

/// `InputEvent` a queue.
pub type InputEvents = VecDeque<InputEvent>;

/// An `InputEvent` is an event which is processed by
/// a `GameElement`.
pub enum InputEvent {
    PlayerMove(Direction),
    EnemyRelease { id: usize },
    GameQuit,
}

/// A `ResultEvent`is an event produced by a `GameElement`
/// as a consequence of processing an `InputElem`.
pub enum ResultEvent {
    NextLevel,
    PlayerDied,
    EnemyDied { id: usize },
    DoNothing,
}

/// It defines the four directions that can be take by the
/// player.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl InputEvent {
    /// Check if the event is to be handle by the player.
    pub fn is_player_event(&self) -> bool {
        match self {
            InputEvent::PlayerMove(_) => true,
            _ => false,
        }
    }

    /// Check if the event is to exit the game.
    pub fn is_quit_event(&self) -> bool {
        match self {
            InputEvent::GameQuit => true,
            _ => false,
        }
    }
}
