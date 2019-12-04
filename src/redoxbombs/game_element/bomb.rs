use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::Fire;
use game_element::GameElement;
use maze::Maze;
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

    /// The time in milliseconds from the bomb creation to the bomb explosion.
    const TIME_TO_EXPLODE: u64 = 3_000;

    /// The duration of the fire created by this bomb.
    const FIRE_DURATION: u64 = 200;

    /// Creates a new `Bomb` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Update the `Bomb` state according to an input event and generate
    /// the right results events.
    pub fn update(&self, maze: &Maze, event: GameEvent, results: &mut VecDeque<ResultEvent>) {
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
                let fire_events = self.set_fire(maze);
                results.extend(fire_events.into_iter());

                // Remove the bomb from the map.
                let explode_event = ResultEvent::BombDelete { id };
                results.push_back(explode_event);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            _ => (),
        }
    }

    /// Create FireNew events for every fire located from the current position
    /// to 2 cells in the directions: up, down, left, right.
    fn set_fire(&self, maze: &Maze) -> VecDeque<ResultEvent> {
        // Get the fire where to create the fires.
        let fire_coordinates = self.compute_fire_coordinates(maze);

        // Convert fire_coordinates to FireNew events.
        fire_coordinates
            .into_iter()
            .map(|coord| {
                let fire = Fire::new(coord.x, coord.y, Bomb::FIRE_DURATION);
                ResultEvent::FireNew { fire }
            })
            .collect()
    }

    /// Compute the coordinates where to create the fires. The fires are created
    /// in the bomb cell and 2 cells in the direction: up, down, left and right,
    /// drawing a cross with center in the bomb cell.
    ///
    /// During the coordinates creation it is check if the cells are blocked
    /// to avoid creating fires transpasing walls.
    fn compute_fire_coordinates(&self, maze: &Maze) -> VecDeque<Coordinates> {
        // Create vector where to store the fire coordinates.
        let mut fire_coordinates = VecDeque::with_capacity(9);

        // Insert the bomb position in the fire vector.
        fire_coordinates.push_back(self.position);

        /// Compute the next coordinate using `next_coordinate_function`
        /// check if the next coordinate is blocked and if not then
        /// store it in the `fire_coordinates` vector.
        macro_rules! insert_next_coordinate_if_not_blocked {
            ($direction:ident, $next_coordinate_function:expr) => {{
                // Compute next coordinate and update coordinate if not blocked.
                $direction = $direction
                    .map(|dir| $next_coordinate_function(&dir))
                    .filter(|dir| !maze.is_blocked(dir.x, dir.y));

                // If not blocked store coordinate in fire_coordinates.
                if let Some(coordinate) = $direction {
                    fire_coordinates.push_back(coordinate);
                }
            }};
        }

        // Init the different fire directions.
        let mut up = Some(self.position);
        let mut down = Some(self.position);
        let mut left = Some(self.position);
        let mut right = Some(self.position);

        // Insert in the fire_coordinates vector every non-blocked position
        // located 1 or 2 positions away from the bomb position, in the
        // directions: up, down, left and right.
        for _ in 0..2 {
            insert_next_coordinate_if_not_blocked!(up, Coordinates::up);
            insert_next_coordinate_if_not_blocked!(down, Coordinates::down);
            insert_next_coordinate_if_not_blocked!(left, Coordinates::left);
            insert_next_coordinate_if_not_blocked!(right, Coordinates::right);
        }

        fire_coordinates
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
