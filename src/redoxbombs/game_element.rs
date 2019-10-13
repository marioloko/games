extern crate termion;

use game_element::termion::raw::IntoRawMode;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Read};

pub type GameElementObject = Box<dyn GameElement>;
pub type GameElementObjects = VecDeque<GameElementObject>;

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

    fn take_turn(&mut self, elems: &GameElementObjects);
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

    fn take_turn(&mut self, elems: &GameElementObjects) {
        let stdin = io::stdin();
        let stdout = io::stdout();

        let stdout_ = stdout.lock().into_raw_mode().unwrap();

        let mut b = [0];
        stdin.lock().read(&mut b).unwrap();

        match b[0] {
            b'h' => self.position = Coordinates { x: self.position.x - 1, y: self.position.y },
            b'j' => self.position = Coordinates { x: self.position.x, y: self.position.y + 1 },
            b'k' => self.position = Coordinates { x: self.position.x, y: self.position.y - 1 },
            b'l' => self.position = Coordinates { x: self.position.x + 1, y: self.position.y },
            _ => {},
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

    fn take_turn(&mut self, elems: &GameElementObjects) {
    }
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

    fn take_turn(&mut self, elems: &GameElementObjects) {
    }
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

    fn take_turn(&mut self, elems: &GameElementObjects) {
    }
}
