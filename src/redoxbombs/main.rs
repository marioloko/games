use std::io::{Read, Write};

struct Coordinates {
    pub x: u32,
    pub y: u32,
}

struct Player {
    position: Coordinates,
    speed: f64,
}

impl Player {
    fn new(x: u32, y: u32) -> Self {
        let position = Coordinates { x, y }
        
        Self {
            position,
            speed:  1.0,
        }
    }
}

trait Enemy {
    fn r#move(&mut self);
}

struct MotionlessEnemy {
    position: Coordinates,
    speed: f64,
}

impl MotionlessEnemy {
    fn new(x: u32, y: u32) -> Self {
        let position = Coordinates { x, y }
        
        Self {
            position,
            speed:  0.0,
        }
    }
}

struct SlowEnemy {
    position: Coordinates,
    speed: f64,
}

impl SlowEnemy {
    fn new(x: u32, y: u32) -> Self {
        let position = Coordinates { x, y }
        
        Self {
            position,
            speed:  0.25,
        }
    }
}

struct Wall {
    position: Coordinates,
    breakable: bool
}

impl Wall {
    fn new(x: u32, y: u32, breakable: bool) -> Self {
        let position = Coordinates { x, y }
        
        Self {
            position,
            breakable,
        }
    }
}

struct Stair {
    position: Coordinates,
}

struct Maze {
    players: Vec<Player>,
    enemies: Vec<Box<dyn Enemy>>,
    walls: Vec<Wall>,
    width: u32,
    map: &'static [u8],
} 

struct Game<R: Read, W: Write> {
    stdin: R,
    stdout: W,
    level: u8,
}

fn main() {
    println!("Hello world in games!");
}
