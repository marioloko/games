use std::fmt;

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

pub trait GameElement {
    fn get_position(&self) -> &Coordinates;
}

#[derive(Debug)]
pub struct Player {
    position: Coordinates,
    speed: f64,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  1.0,
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
    speed: f64,
}

impl MotionlessEnemy {
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  0.0,
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
    speed: f64,
}

impl SlowEnemy {
    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  0.25,
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
