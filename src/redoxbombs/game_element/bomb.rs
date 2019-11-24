use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::Fire;
use game_element::GameElement;
use std::collections::VecDeque;

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
    const TIME_TO_EXPLODE: u64 = 3_000;

    /// Creates a new `Bomb` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Update the `Bomb` state according to an input event and generate
    /// the right results events.
    pub fn update(&self, event: GameEvent, results: &mut VecDeque<ResultEvent>) {
        match event {
            GameEvent::BombInit { id } => {
                // Set bomb to explode in the future.
                let explode_event = GameEvent::BombExplode { id };
                let explode_scheduling_event = ResultEvent::GameScheduleEvent {
                    millis: Self::TIME_TO_EXPLODE,
                    event: explode_event,
                };
                results.push_back(explode_scheduling_event);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            GameEvent::BombExplode { id } => {
                // Create the bomb explosion fires.
                let fires = self.set_fire();
                let result = ResultEvent::BombExplode { id, fires };
                results.push_back(result);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            _ => (),
        }
    }

    /// Create fires in 2 cells cross fashion.
    fn set_fire(&self) -> Vec<Fire> {
        // Get all the surrounded cells.
        let up = self.position.up();
        let down = self.position.down();
        let left = self.position.left();
        let right = self.position.right();

        let fire_coordinates = vec![
            // Current cell position.
            self.position,
            // Surrounding cells.
            up,
            down,
            left,
            right,
            // Surrounding cells one step further away.
            up.up(),
            down.down(),
            left.left(),
            right.right(),
        ];

        // The fire duration.
        let duration = 1000;

        // Create the fires with different coordinates and starting time.
        fire_coordinates
            .into_iter()
            .enumerate()
            .map(|(idx, coord)| {
                let x = coord.x;
                let y = coord.y;
                let start_after = (duration / 4) * (idx / 5);
                Fire::new(x, y, start_after, duration)
            })
            .collect()
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
