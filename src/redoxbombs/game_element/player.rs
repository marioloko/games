use events::{Direction, InputEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use game_element::Bomb;
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

    /// Handle the player `InputEvent` and produce the appropriated `ResultEvent`.
    pub fn take_turn(&mut self, maze: &Maze, event: InputEvent) -> ResultEvent {
        match event {
            InputEvent::PlayerMove(dir) => self.move_player(maze, dir),
            InputEvent::PlayerCreateBomb => self.put_bomb(),
            _ => ResultEvent::DoNothing,
        }
    }

    /// Move the player toward the given direction if the tile is not blocked.
    fn move_player(&mut self, maze: &Maze, dir: Direction) -> ResultEvent {
        // Compute next position to move.
        let next_position = match dir {
            Direction::Up => self.position.up(),
            Direction::Down => self.position.down(),
            Direction::Left => self.position.left(),
            Direction::Right => self.position.right(),
        };

        // Move to next position if not blocked.
        if !maze.is_blocked(next_position.x, next_position.y) {
            self.position = next_position;
        }

        ResultEvent::DoNothing
    }

    /// Generate a `ResultEvent` containing a bomb located at the 
    /// player position.
    fn put_bomb(&self) -> ResultEvent {
        let position = self.get_position();
        let (x, y) = (position.x, position.y);

        let bomb = Bomb::new(x, y);

        ResultEvent::BombCreated { bomb }
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
