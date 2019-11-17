use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;

/// A `Fire` object represents the explosion fire produced by a bomb.
#[derive(Debug)]
pub struct Fire {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,

    /// Milliseconds to start the fire.
    start_after: usize,

    /// Duration in milliseconds before extinguish the fire.
    duration: usize,
}

impl Fire {
    /// The name used for the parser to identify the an `Bomb` object.
    pub(super) const NAME: &'static str = "Fire";

    /// Character to represent an `Bomb` object in the `Maze`.
    const REPRESENTATION: char = '*';

    /// Creates a new `Bomb` object given its coordinates, the starting time
    /// and its duration.
    pub fn new(x: usize, y: usize, start_after: usize, duration: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position , start_after, duration }
    }

    /// Get the milliseconds to start the fire.
    pub fn start_after(&self) -> usize {
        self.start_after
    }

    /// Get the milliseconds before putting the fire out.
    pub fn duration(&self) -> usize {
        self.duration
    }

    /// Take a turn given an input event and return a result event as a result.
    pub fn take_turn(&self, event: GameEvent) -> ResultEvent {
        match event {
            GameEvent::FireInit { id } => ResultEvent::FireCheckCollision { id },
            GameEvent::FireCheckCollision { id } => ResultEvent::FireCheckCollision { id },
            GameEvent::FirePutOut { id } => ResultEvent::FirePutOut { id },
            _ => ResultEvent::DoNothing,
        }
    }
}

impl GameElement for Fire {
    /// Return the current possition of the game `Bomb`.
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    /// Return the `Bomb` representation.
    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }
}
