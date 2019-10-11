//use std::io::{Read, Write};
mod maze;
mod game_element;

use maze::Maze;
use game_element::{GameElement, Player, Enemy, SlowEnemy, MotionlessEnemy};

const MAP_1: &'static [u8] = include_bytes!("map1.txt");

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
