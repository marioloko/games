use events::{Direction, InputEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use maze::Maze;

/// Represent the Player.
#[derive(Debug)]
pub struct Player {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,
}

impl Player {
    /// The name used for the parser to identify the an `Player` object.
    pub(super) const NAME: &'static str = "Player";

    /// Character to represent an `Player` object in the `Maze`.
    const REPRESENTATION: char = '@';

    /// Creates a new `Player` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Take a turn given an input event and return a result event as a
    /// result.
    pub fn take_turn(&mut self, maze: &Maze, event: InputEvent) -> ResultEvent {
        let next_position = match event {
            InputEvent::PlayerMove(dir) => match dir {
                Direction::Up => self.position.up(),
                Direction::Down => self.position.down(),
                Direction::Left => self.position.left(),
                Direction::Right => self.position.right(),
            },
            _ => self.position,
        };

        // Check if computed position is blocked.
        if !maze.is_blocked(next_position.x, next_position.y) {
            self.position = next_position;
        }

        ResultEvent::DoNothing
    }
}

impl GameElement for Player {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }
}
