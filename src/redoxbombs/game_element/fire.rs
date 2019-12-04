use events::{GameEvent, ResultEvent};
use game_element::Coordinates;
use game_element::Enemy;
use game_element::GameElement;
use game_element::Player;
use std::collections::VecDeque;

/// A `Fire` object represents the explosion fire produced by a bomb.
#[derive(Debug)]
pub struct Fire {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,

    /// Duration in milliseconds before extinguish the fire.
    duration: u64,
}

impl Fire {
    /// The name used for the parser to identify the an `Bomb` object.
    pub(super) const NAME: &'static str = "Fire";

    /// Character to represent an `Bomb` object in the `Maze`.
    const REPRESENTATION: char = '*';

    /// Creates a new `Bomb` object given its coordinates, the starting time
    /// and its duration.
    pub fn new(x: usize, y: usize, duration: u64) -> Self {
        let position = Coordinates { x, y };

        Self { position, duration }
    }

    /// Update the `Fire` state according to an input event and generate
    /// the right results events.
    pub fn update(
        &self,
        player: &Player,
        enemies: &[Option<Enemy>],
        event: GameEvent,
        results: &mut VecDeque<ResultEvent>,
    ) {
        match event {
            GameEvent::FireInit { id } => {
                // Add a fire extintion event to put out the fire after `duration`.
                let extintion_event = GameEvent::FirePutOut { id };
                let extintion_event = ResultEvent::GameScheduleEvent {
                    millis: self.duration,
                    event: extintion_event,
                };
                results.push_back(extintion_event);

                // Check collisions with other game elements.
                let collision_event = GameEvent::FireCheckCollision { id };
                let collision_event = ResultEvent::GameSetEvent {
                    event: collision_event,
                };
                results.push_back(collision_event);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            GameEvent::FireCheckCollision { id }
                if player.get_position() == self.get_position() =>
            {
                // Kill the player if burnt by fire.
                let player_died_event = ResultEvent::PlayerDied;
                results.push_back(player_died_event);
            }
            GameEvent::FireCheckCollision { id } => {
                // Kill enemy if collides with any.
                let kill_enemy_events = self.create_kill_enemy_event(enemies);
                results.extend(kill_enemy_events.into_iter());

                // Recheck collision with other game elements.
                let collision_event = ResultEvent::GameSetEvent { event };
                results.push_back(collision_event);
            }
            GameEvent::FirePutOut { id } => {
                // Extinguish the fire.
                let result = ResultEvent::FireDelete { id };
                results.push_back(result);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            _ => (),
        }
    }

    /// Create a `EnemyDelete` event for every enemy who collides with the fire.
    fn create_kill_enemy_event(&self, enemies: &[Option<Enemy>]) -> Vec<ResultEvent> {
        enemies
            .iter()
            .enumerate()
            .filter(|(_, enemy)| self.enemy_collides(enemy))
            .map(|(id, _)| ResultEvent::EnemyDelete { id })
            .collect()
    }

    /// Checks whether or not a given `Option<Enemy>` is in the same
    /// cell as the fire.
    fn enemy_collides(&self, enemy: &Option<Enemy>) -> bool {
        if let Some(enemy) = enemy {
            return enemy.get_position() == self.get_position();
        }

        false
    }
}

impl GameElement for Fire {
    /// Return the current possition of the game `Bomb`.
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    /// Return the `Bomb` representation.
    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }
}
