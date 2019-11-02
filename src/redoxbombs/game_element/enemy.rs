use events::{InputEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use game_element::Player;
use maze::Maze;
use rand::{self, Rng};

/// A `Enemy` object represents a game enemy.
#[derive(Debug)]
pub struct Enemy {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,
}

impl Enemy {
    /// The name used for the parser to identify the an `Enemy` object.
    pub(super) const NAME: &'static str = "Enemy";

    /// Character to represent an `Enemy` object in the `Maze`.
    const REPRESENTATION: char = 'E';

    /// Creates a new `Enemy` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Take a turn given an input event and return a result event as a
    /// result.
    pub fn take_turn(&mut self, player: &Player, maze: &Maze, event: InputEvent) -> ResultEvent {
        let result_event = match event {
            InputEvent::EnemyRelease { id } => ResultEvent::EnemyBlock { id },
            _ => return ResultEvent::DoNothing,
        };

        let mut directions = vec![
            self.position.up(),
            self.position.down(),
            self.position.left(),
            self.position.right(),
            self.position,
        ];

        // Unorder them to increase movement randomness
        rand::thread_rng().shuffle(&mut directions);

        let next_position = directions
            .into_iter()
            .filter(|dir| !maze.is_blocked(dir.x, dir.y))
            .map(|dir| {
                let dist = dir.manhattan_distance(player.get_position());
                (dir, dist)
            })
            .max_by(|(_, dist1), (_, dist2)| dist2.partial_cmp(dist1).unwrap())
            .map(|(dir, _)| dir)
            .unwrap();

        self.position = next_position;

        result_event
    }
}

impl GameElement for Enemy {
    /// Return the current possition of the game `Enemy`.
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    /// Return the `Enemy` representation.
    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }
}
