use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
enum GameElementType {
    Player,
    Enemy,
    Stairs,
}

pub trait GameElement: fmt::Debug {
    fn get_position(&self) -> &Coordinates;

    fn get_type(&self) -> GameElementType;

    fn take_turn(self, elems: &mut VecDeque<Box<dyn GameElement>>) -> Option<Box<dyn GameElement>>;
}

#[derive(Debug)]
pub struct Player {
    position: Coordinates,
}

impl Player {
    pub const NAME: &'static str = "Player";
    const TYPE: GameElementType = GameElementType::Player;
    const SPEED: f64 = 1.0;

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

    fn take_turn(self, elems: &mut VecDeque<Box<dyn GameElement>>) -> Option<Box<dyn GameElement>> {
        None
    }
}

#[derive(Debug)]
pub struct MotionlessEnemy {
    position: Coordinates,
}

impl MotionlessEnemy {
    pub const NAME: &'static str = "MotionlessEnemy";
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.0;

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

    fn take_turn(self, elems: &mut VecDeque<Box<dyn GameElement>>) -> Option<Box<dyn GameElement>> {
        None
    }
}

#[derive(Debug)]
pub struct SlowEnemy {
    position: Coordinates,
}

impl SlowEnemy {
    pub const NAME: &'static str = "SlowEnemy";
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.25;

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

    fn take_turn(self, elems: &mut VecDeque<Box<dyn GameElement>>) -> Option<Box<dyn GameElement>> {
        None
    }
}

#[derive(Debug)]
pub struct Stairs {
    position: Coordinates,
}

impl Stairs {
    pub const NAME: &'static str = "Stairs";
    const TYPE: GameElementType = GameElementType::Stairs;

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

    fn take_turn(self, elems: &mut VecDeque<Box<dyn GameElement>>) -> Option<Box<dyn GameElement>> {
        None
    }
}
