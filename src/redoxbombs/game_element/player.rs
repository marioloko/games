use events::{Direction, GameEvent, InputEvent, ResultEvent};
use game_element::Bomb;
use game_element::Coordinates;
use game_element::{AnyGameElementAt, GameElement};
use maze::Maze;
use std::collections::VecDeque;

/// Represent the Player.
#[derive(Debug)]
pub struct Player {
    /// The `Coordinates` where this game element is located.
    position: Coordinates,

    /// Maximum number of simultaneous `Bomb` allowed for the `Player`.
    max_bombs: usize,

    /// Number of simultaneos bombs that the `Player` can put at this moment.
    bombs: usize,

    /// Milliseconds for the `Player` to increment the number of available
    /// `Bomb`s.
    bomb_recovery_millis: u64,
}

impl Player {
    /// The name used for the parser to identify the an `Player` object.
    pub(super) const NAME: &'static str = "Player";

    /// Character to represent an `Player` object in the `Maze`.
    const REPRESENTATION: char = '@';

    /// Initial maximum number of player simultaneous bombs.
    const INITIAL_MAX_BOMBS: usize = 3;

    /// Initial time needed to increment the player number of bombs
    /// in milliseconds.
    const INITIAL_BOMB_RECOVERY_MILLIS: u64 = 4_000;

    /// Creates a new `Player` object given its coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        // Player is created at the given coordinates.
        let position = Coordinates { x, y };

        // Set the maximum number of bombs.
        let max_bombs = Self::INITIAL_MAX_BOMBS;

        // At the begining the player has the maximum number
        // of bombs allowed.
        let bombs = max_bombs;

        // Set the bomb recovery time
        let bomb_recovery_millis = Self::INITIAL_BOMB_RECOVERY_MILLIS;

        // Create Player.
        Self {
            position,
            max_bombs,
            bombs,
            bomb_recovery_millis,
        }
    }

    /// Update the `Player` state according to an `InputEvent` and generate
    /// the right `ResultEvent`.
    pub fn update_from_input_event(
        &mut self,
        bombs: &[Option<Bomb>],
        maze: &Maze,
        event: InputEvent,
        results: &mut VecDeque<ResultEvent>,
    ) {
        match event {
            InputEvent::PlayerMove(dir) => {
                // Move player in the input direction.
                self.move_player(maze, bombs, dir);

                // Notify that the game state has changed.
                let updated_event = ResultEvent::GameUpdated;
                results.push_back(updated_event);
            }
            InputEvent::PlayerCreateBomb => {
                // Check if the user has bombs.
                if self.bombs > 0 {
                    // Put a bomb in the current position.
                    let bomb = self.put_bomb();
                    let result = ResultEvent::BombNew { bomb };
                    results.push_back(result);

                    // The player loses a bomb.
                    self.bombs -= 1;

                    // Schedule bomb to recover after the recover time.
                    let bomb_recover_event = ResultEvent::GameScheduleEvent {
                        millis: self.bomb_recovery_millis,
                        event: GameEvent::PlayerRecoverBomb,
                    };
                    results.push_back(bomb_recover_event);
                }
            }
            _ => (),
        }
    }

    /// Update the `Player` state according to a game event.
    pub fn update(&mut self, event: GameEvent) {
        match event {
            GameEvent::PlayerRecoverBomb => {
                // Check if user has maximum number of bombs.
                if self.bombs < self.max_bombs {
                    // Recover one bomb.
                    self.bombs += 1;
                }
            }
            _ => (),
        }
    }

    /// Move the player toward the given direction if the tile is not blocked.
    fn move_player(&mut self, maze: &Maze, bombs: &[Option<Bomb>], dir: Direction) {
        // Compute next position to move.
        let next_position = match dir {
            Direction::Up => self.position.up(),
            Direction::Down => self.position.down(),
            Direction::Left => self.position.left(),
            Direction::Right => self.position.right(),
        };

        // Check if the position is blocked by any bomb.
        let blocked_by_bombs = bombs.any_game_element_at(&next_position);

        // Move to next position if not blocked.
        if !maze.is_blocked(next_position.x, next_position.y) && !blocked_by_bombs {
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
