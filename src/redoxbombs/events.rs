use std::collections::VecDeque;

pub type InputEvents = VecDeque<InputEvent>;

pub enum InputEvent {
    PlayerMove(Direction),
    EnemyRelease { id: usize },
    GameQuit,
}

pub enum ResultEvent {
    NextLevel,
    PlayerDied,
    EnemyDied { id: usize },
    DoNothing,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl InputEvent {
    pub fn is_player_event(&self) -> bool {
        match self {
            InputEvent::PlayerMove(_) => true,
            _ => false,
        }
    }
}
