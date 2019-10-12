use std::fmt;

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

pub trait GameElement: FromStr {
    fn get_position(&self) -> &Coordinates;
}

#[derive(Debug)]
pub struct Player {
    position: Coordinates,
}

impl Player {
    const SPEED = 1.0;

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
}

pub trait Enemy: fmt::Debug + GameElement  {
    fn r#move(&mut self);
}

#[derive(Debug)]
pub struct MotionlessEnemy {
    position: Coordinates,
}

impl MotionlessEnemy {
    const SPEED = 0.0;

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
    const SPEED = 0.25;

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
}

impl Enemy for SlowEnemy {
    fn r#move(&mut self) {
    }
}


struct Stair {
    position: Coordinates,
}

impl GameElement for Stair {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }
}
