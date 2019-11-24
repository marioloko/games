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
    start_after: u64,

    /// Duration in milliseconds before extinguish the fire.
    duration: u64,
}

impl Fire {
    /// The name used for the parser to identify the an `Bomb` object.
    pub(super) const NAME: &'static str = "Fire";

    /// Character to represent an `Bomb` object in the `Maze`.
    const REPRESENTATION: char = '*';

    /// Creates a new `Bomb` object given its coordinates, the starting time
    /// and its duration.
    pub fn new(x: usize, y: usize, start_after: u64, duration: u64) -> Self {
        let position = Coordinates { x, y };

        Self {
            position,
            start_after,
            duration,
        }
    }

    /// Get the milliseconds to start the fire.
    pub fn start_after(&self) -> u64 {
        self.start_after
    }

    /// Update the `Fire` state according to an input event and generate
    /// the right results events.
    pub fn update(&self, event: GameEvent, results: &mut VecDeque<ResultEvent>) {
        match event {
            GameEvent::FireInit { id } => {
                // Add a fire extintion event to put out the fire after `duration`.
                let extintion_event = GameEvent::FirePutOut { id };
                let extintion_event = ResultEvent::GameScheduleEvent {
                    millis: self.duration,
                    event: extintion_event,
                };
                results.push_back(extintion_event);

                // Check collisions with other game elements.
                let collision_event = GameEvent::FireCheckCollision { id };
                let collision_event = ResultEvent::GameSetEvent {
                    event: collision_event,
                };
                results.push_back(collision_event);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            GameEvent::FireCheckCollision { id } => {
                // Recheck collision with other game elements.
                let collision_event = ResultEvent::GameSetEvent { event };
                results.push_back(collision_event);
            }
            GameEvent::FirePutOut { id } => {
                // Extinguish the fire.
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
