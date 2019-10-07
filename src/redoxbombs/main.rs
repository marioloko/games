use std::io::{Read, Write};

struct Coordinates {
    x: u32,
    y: u32,
}

struct Player {
    position: Coordinates,
    speed: f64,
}

trait Enemy {
    fn move(&mut self);
}

struct MotionlessEnemy {
    position: Coordinates,
    speed: f64,
}

struct Wall {
    position: Coordinates,
}

struct BreakableWall {
    position: Coordinates,
    broken: bool,
}

struct Stair {
    position: Coordinates,
}

struct Maze {
    players: Vec<Player>,
    enemies: Vec<dyn Enemy>,
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
