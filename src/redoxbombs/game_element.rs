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

pub trait GameElement {
    fn get_position(&self) -> &Coordinates;

    fn get_type(&self) -> GameElementType;
}

#[derive(Debug)]
pub struct Player {
    position: Coordinates,
}

impl Player {
    const TYPE: GameElementType  = GameElementType::Player;
    const SPEED: f64 = 1.0;

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
        }
    }
}

impl GameElement for Player {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }
}

pub trait Enemy: fmt::Debug + GameElement  {
    fn r#move(&mut self);
}

#[derive(Debug)]
pub struct MotionlessEnemy {
    position: Coordinates,
}

impl MotionlessEnemy {
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.0;

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
        }
    }
}

impl GameElement for MotionlessEnemy {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }
}

impl Enemy for MotionlessEnemy {
    fn r#move(&mut self) {
    }
}


#[derive(Debug)]
pub struct SlowEnemy {
    position: Coordinates,
}

impl SlowEnemy {
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.25;

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
        }
    }
}

impl GameElement for SlowEnemy {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }
}

impl Enemy for SlowEnemy {
    fn r#move(&mut self) {
    }
}


struct Stairs {
    position: Coordinates,
}

impl Stairs {
    const TYPE: GameElementType = GameElementType::Stairs;
}

impl GameElement for Stairs {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }
}
