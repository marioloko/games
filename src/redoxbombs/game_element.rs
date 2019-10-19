use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Read};
use termion::raw::IntoRawMode;
use maze::Maze;

pub type GameElementObject<'a> = Box<dyn GameElement + 'a>;
pub type GameElementObjects<'a> = VecDeque<GameElementObject<'a>>;

#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum GameElementType {
    Player,
    Enemy,
    Stairs,
}

pub trait GameElement: fmt::Debug {
    fn get_position(&self) -> &Coordinates;

    fn get_type(&self) -> GameElementType;

    fn get_representation(&self) -> char;

    fn take_turn(&mut self, elems: &GameElementObjects, maze: &Maze);
}

pub fn generate_game_element(name: &str, x: usize, y: usize) -> GameElementObject {
    match name {
        Player::NAME => Box::new(Player::new(x, y)),
        MotionlessEnemy::NAME => Box::new(MotionlessEnemy::new(x, y)),
        SlowEnemy::NAME => Box::new(SlowEnemy::new(x, y)),
        Stairs::NAME => Box::new(Stairs::new(x, y)),
        _ => panic!("Unrecognized game element: {}", name),
    }
}

#[derive(Debug)]
struct Player {
    position: Coordinates,
}

impl Player {
    const NAME: &'static str = "Player";
    const TYPE: GameElementType = GameElementType::Player;
    const SPEED: f64 = 1.0;
    const REPRESENTATION: char = '@';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for Player {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(&mut self, elems: &GameElementObjects, maze: &Maze) {
        let stdin = io::stdin();

        let mut b = [0];
        stdin.lock().read(&mut b).unwrap();

        
        let next = match b[0] {
            b'h' => {
                Coordinates {
                    x: self.position.x - 1,
                    y: self.position.y,
                }
            }
            b'j' => {
                Coordinates {
                    x: self.position.x,
                    y: self.position.y + 1,
                }
            }
            b'k' => {
                Coordinates {
                    x: self.position.x,
                    y: self.position.y - 1,
                }
            }
            b'l' => {
                Coordinates {
                    x: self.position.x + 1,
                    y: self.position.y,
                }
            }
            _ => self.position
        };

        if !maze.is_blocked(next.x, next.y) {
            self.position = next;
        }
    }
}

#[derive(Debug)]
struct MotionlessEnemy {
    position: Coordinates,
}

impl MotionlessEnemy {
    const NAME: &'static str = "MotionlessEnemy";
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.0;
    const REPRESENTATION: char = 'M';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for MotionlessEnemy {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(&mut self, _elems: &GameElementObjects, _maze: &Maze) {}
}

#[derive(Debug)]
struct SlowEnemy {
    position: Coordinates,
}

impl SlowEnemy {
    const NAME: &'static str = "SlowEnemy";
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.25;
    const REPRESENTATION: char = 'S';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for SlowEnemy {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(&mut self, elems: &GameElementObjects, maze: &Maze) {}
}

#[derive(Debug)]
struct Stairs {
    position: Coordinates,
}

impl Stairs {
    const NAME: &'static str = "Stairs";
    const TYPE: GameElementType = GameElementType::Stairs;
    const REPRESENTATION: char = '%';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for Stairs {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(&mut self, elems: &GameElementObjects, maze: &Maze) {}
}
