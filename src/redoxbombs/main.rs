//use std::io::{Read, Write};
use std::fmt;
mod maze;
use maze::Maze;

const MAP_1: &'static [u8] = include_bytes!("map1.txt");

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
struct Player {
    position: Coordinates,
    speed: f64,
}

impl Player {
    fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  1.0,
        }
    }
}

trait Enemy: fmt::Debug {
    fn r#move(&mut self);
}

#[derive(Debug)]
struct MotionlessEnemy {
    position: Coordinates,
    speed: f64,
}

impl MotionlessEnemy {
    fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  0.0,
        }
    }
}

impl Enemy for MotionlessEnemy {
    fn r#move(&mut self) {
    }
}

#[derive(Debug)]
struct SlowEnemy {
    position: Coordinates,
    speed: f64,
}

impl SlowEnemy {
    fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };
        
        Self {
            position,
            speed:  0.25,
        }
    }
}

impl Enemy for SlowEnemy {
    fn r#move(&mut self) {
    }
}


struct Stair {
    position: Coordinates,
}


struct Game {
//    stdin: R,
//    stdout: W,
    maze: Maze,
    players: Vec<Player>,
    enemies: Vec<Box<dyn Enemy>>,
    level: u8,
}

impl Game {
    fn load_characters(map: &[u8]) {
        let width = map.iter().take_while(|&b| *b != b'\n').count();

        let flat_map: Vec<&u8> = map.iter().filter(|&b| *b != b'\n').collect();

        let mut players = Vec::new();
        let mut enemies: Vec<Box<dyn Enemy>> = Vec::new();

        for (idx, &elem) in flat_map.iter().enumerate() {
            let x = idx % width;
            let y = idx / width;
            match *elem {
                b'@' => players.push(Player::new(x, y)),
                b'M' => enemies.push(Box::new(MotionlessEnemy::new(x, y))),
                b'S' => enemies.push(Box::new(SlowEnemy::new(x, y))),
                _ => (),
            }
        }
        println!("{:?}", players);
        println!("{:?}", enemies);
    }
}

fn main() {
    println!("{}", Maze::from(MAP_1));
    Game::load_characters(MAP_1);
}
