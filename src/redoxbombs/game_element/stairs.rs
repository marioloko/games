use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use game_element::Player;
use maze::Maze;

/// A `Stairs` object represents an object used to
/// go to the next level.
#[derive(Debug)]
pub struct Stairs {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,
}

impl Stairs {
    /// The name used for the parser to identify the an `Stairs` object.
    pub(super) const NAME: &'static str = "Stairs";

    /// Character to represent an `Stair` object in the `Maze`.
    const REPRESENTATION: char = '%';

    /// Creates a new `Stairs` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Take a turn given a game event and return a result event as a
    /// result.
    pub fn take_turn(&mut self, player: &Player, maze: &Maze, event: GameEvent) -> ResultEvent {
        match event {
            GameEvent::StairsRelease 
                if player.get_position() == self.get_position() => ResultEvent::NextLevel,
            _ => ResultEvent::StairsBlock,
        }
    }
}

impl GameElement for Stairs {
    /// Return the current possition of the game `Stairs`.
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    /// Return the `Stairs` representation.
    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }
}
