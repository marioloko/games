use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use game_element::Player;
use maze::Maze;
use std::collections::VecDeque;

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

    /// Update the `Player` state according to an input event and generate
    /// the right results events.
    pub fn update(
        &mut self,
        player: &Player,
        maze: &Maze,
        event: GameEvent,
        results: &mut VecDeque<ResultEvent>,
    ) {
        match event {
            GameEvent::StairsRelease if player.get_position() == self.get_position() => {
                let result = ResultEvent::NextLevel;
                results.push_back(result);
            }
            _ => {
                let result = ResultEvent::StairsBlock;
                results.push_back(result);
            }
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
