//use std::io::{Read, Write};
mod maze;
use maze::Maze;

const MAP_1: &'static [u8] = include_bytes!("map1.txt");

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
        let position = Coordinates { x, y };
        
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
        let position = Coordinates { x, y };
        
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
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  0.25,
        }
    }
}



struct Stair {
    position: Coordinates,
}


//struct Game<R: Read, W: Write> {
//    stdin: R,
//    stdout: W,
//    level: u8,
//}

fn main() {
    println!("{}", Maze::from(MAP_1));
}
