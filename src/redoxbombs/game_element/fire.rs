use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use std::collections::VecDeque;

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

        Self {
            position,
            start_after,
            duration,
        }
    }

    /// Get the milliseconds to start the fire.
    pub fn start_after(&self) -> usize {
        self.start_after
    }

    /// Get the milliseconds before putting the fire out.
    pub fn duration(&self) -> usize {
        self.duration
    }

    /// Update the `Fire` state according to an input event and generate
    /// the right results events.
    pub fn update(&self, event: GameEvent, results: &mut VecDeque<ResultEvent>) {
        match event {
            GameEvent::FireInit { id } => {
                let result = ResultEvent::FireCheckCollision { id };
                results.push_back(result);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            GameEvent::FireCheckCollision { id } => {
                let result = ResultEvent::FireCheckCollision { id };
                results.push_back(result);
            }
            GameEvent::FirePutOut { id } => {
                let result = ResultEvent::FirePutOut { id };
                results.push_back(result);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            _ => (),
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
