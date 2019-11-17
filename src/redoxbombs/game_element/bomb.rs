use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use game_element::Fire;

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
            GameEvent::BombExplode { id } => {
                let fires = self.set_fire();
                ResultEvent::BombExplode { id, fires }
            }
            GameEvent::BombInit { id } => ResultEvent::BombInit { id },
            _ => ResultEvent::DoNothing,
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
        fire_coordinates.into_iter().enumerate().map(|(idx, coord)| {
            let x = coord.x;
            let y = coord.y;
            let start_after = (duration / 4) * (idx / 5);
            Fire::new(x, y, start_after, duration)
        }).collect()
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
