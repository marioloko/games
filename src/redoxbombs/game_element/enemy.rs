use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::GameElement;
use game_element::Player;
use maze::Maze;
use rand::{self, Rng};
use std::collections::VecDeque;

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

    /// It is the delay before the next enemy movement. It is useful to
    /// slow down the enemies to a speed manageable by an human player.
    const MOVEMENT_DELAY: u64 = 500;

    /// Creates a new `Enemy` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }

    /// Update the `Enemy` state according to an input event and generate
    /// the right results events.
    pub fn update(
        &mut self,
        player: &Player,
        maze: &Maze,
        event: GameEvent,
        results: &mut VecDeque<ResultEvent>,
    ) {
        match event {
            GameEvent::EnemyInit { id } => {
                // Let the enemy to start moving.
                let movement_event = GameEvent::EnemyMove { id };
                let movement_event = ResultEvent::GameSetEvent {
                    event: movement_event,
                };
                results.push_back(movement_event);

                // Let the enemy to start checking their collisions.
                let collision_event = GameEvent::EnemyCheckCollision { id };
                let collision_event = ResultEvent::GameSetEvent {
                    event: collision_event,
                };
                results.push_back(collision_event);
            }
            GameEvent::EnemyMove { id } => {
                // Move the enemy towards the player.
                self.move_towards(player, maze);

                // Reschedule this event to be executed in the future after a delay.
                let scheduling_event = ResultEvent::GameScheduleEvent {
                    millis: Self::MOVEMENT_DELAY,
                    event,
                };
                results.push_back(scheduling_event);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            GameEvent::EnemyCheckCollision { id }
                if self.get_position() == player.get_position() =>
            {
                // If enemy collides with player then the player dies.
                let result = ResultEvent::PlayerDied;
                results.push_back(result);
            }
            GameEvent::EnemyCheckCollision { id } => {
                // Ensure to recheck the collision again in the future.
                let result = ResultEvent::GameSetEvent { event };
                results.push_back(result);
            }
            _ => (),
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
