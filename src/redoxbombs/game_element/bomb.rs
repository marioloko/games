use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;

/// A `Bomb` object represents a non exploded bomb.
#[derive(Debug)]
pub struct Bomb {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,
}

impl Bomb {
    /// The name used for the parser to identify the an `Bomb` object.
    pub(super) const NAME: &'static str = "Bomb";

    /// Character to represent an `Bomb` object in the `Maze`.
    const REPRESENTATION: char = 'o';

    /// Creates a new `Bomb` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Take a turn given an input event and return a result event as a result.
    pub fn take_turn(&self, event: GameEvent) -> ResultEvent {
        match event {
            GameEvent::BombExplode { id } => ResultEvent::BombExplode { id },
            GameEvent::BombInit { id } => ResultEvent::BombInit { id },
            _ => ResultEvent::DoNothing,
        }
    }
}

impl GameElement for Bomb {
    /// Return the current possition of the game `Bomb`.
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    /// Return the `Bomb` representation.
    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }
}
