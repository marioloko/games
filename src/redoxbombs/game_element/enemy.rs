use events::{GameEvent, ResultEvent};
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

    /// Take a turn given an input event and return a result event as a result.
    pub fn take_turn(&mut self, player: &Player, maze: &Maze, event: GameEvent) -> ResultEvent {
        let result_event = match event {
            GameEvent::EnemyRelease { id } => ResultEvent::EnemyBlock { id },
            _ => return ResultEvent::DoNothing,
        };

        // Move the enemy towards the next non-blocked position closer to the player.
        self.move_towards(player, maze);

        result_event
    }

    /// Checks if the enemy and the player has collided, and if so, kill the player.
    pub fn check_collision(&mut self, player: &Player, game_event: GameEvent) -> ResultEvent {
        match game_event {
            GameEvent::EnemyCheckCollision { id }
                if self.get_position() == player.get_position() => ResultEvent::PlayerDied,
            GameEvent::EnemyCheckCollision { id } => ResultEvent::EnemyCheckCollision { id },
            _ => ResultEvent::DoNothing,
        }
    }

    /// Move the enemy towards the next non-blocked position closer to the player.
    fn move_towards(&mut self, player: &Player, maze: &Maze) {
        // Compute all the possible positions to move.
        let mut directions = vec![
            self.position.up(),
            self.position.down(),
            self.position.left(),
            self.position.right(),
            self.position,
        ];

        // Unorder them to increase movement randomness.
        rand::thread_rng().shuffle(&mut directions);

        // Select the next position.
        let next_position = directions
            .into_iter()
            // Discard blocked positions.
            .filter(|dir| !maze.is_blocked(dir.x, dir.y))
            // Compute the manhattan distance to the player for every
            // non-blocked surrounded positions.
            .map(|dir| {
                let dist = dir.manhattan_distance(player.get_position());
                (dir, dist)
            })
            // Get the position whose distance is the lesser as the next
            // position.
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            // Extract the coordinates of the chosen position and discard
            // the distance.
            .map(|(dir, _)| dir)
            .unwrap();

        // Update the position to the next non-blocked position
        // closer to the player.
        self.position = next_position;
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
