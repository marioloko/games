use events::{Direction, InputEvent, ResultEvent};
use game_element::Bomb;
use game_element::Coordinates;
use game_element::GameElement;
use maze::Maze;
use std::collections::VecDeque;

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

    /// Update the `Player` state according to an input event and generate
    /// the right results events.
    pub fn update(&mut self, maze: &Maze, event: InputEvent, results: &mut VecDeque<ResultEvent>) {
        match event {
            InputEvent::PlayerMove(dir) => {
                self.move_player(maze, dir);
            }
            InputEvent::PlayerCreateBomb => {
                let bomb = self.put_bomb();
                let result = ResultEvent::BombCreated { bomb };
                results.push_back(result);
            }
            _ => (),
        }
    }

    /// Move the player toward the given direction if the tile is not blocked.
    fn move_player(&mut self, maze: &Maze, dir: Direction) {
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
    }

    /// Generate a `ResultEvent` containing a bomb located at the
    /// player position.
    fn put_bomb(&self) -> Bomb {
        let position = self.get_position();
        let (x, y) = (position.x, position.y);

        Bomb::new(x, y)
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
