//use std::io::{Read, Write};
mod game_element;
mod maze;

use game_element::{
    GameElementObject, GameElementObjects, MotionlessEnemy, Player, SlowEnemy, Stairs,
};
use maze::Maze;
use std::collections::VecDeque;

const MAP_1: &'static [u8] = include_bytes!("map1.txt");
const GAME_ELEMENTS_1: &'static str = include_str!("game_elements1.txt");

#[derive(Debug)]
struct Game {
    //    stdin: R,
    //    stdout: W,
    maze: Maze,
    game_elements: GameElementObjects,
    level: u8,
}

impl Game {
    fn new(map: &[u8], game_elements: &str) -> Game {
        let maze = Maze::from(map);
        let game_elements = Game::load_game_elements(game_elements);

        Game {
            maze,
            game_elements,
            level: 1,
        }
    }

    fn load_game_elements(game_elements: &str) -> GameElementObjects {
        let game_elements = game_elements.lines();

        let mut result: GameElementObjects = VecDeque::new();
        for game_element in game_elements {
            let game_element: Vec<&str> = game_element.split(' ').collect();
            let (name, x, y) = (game_element[0], game_element[1], game_element[2]);

            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();

            let game_element: GameElementObject = match name {
                Player::NAME => Box::new(Player::new(x, y)),
                MotionlessEnemy::NAME => Box::new(MotionlessEnemy::new(x, y)),
                SlowEnemy::NAME => Box::new(SlowEnemy::new(x, y)),
                Stairs::NAME => Box::new(Stairs::new(x, y)),
                _ => panic!("Unrecognized game element: {}", name),
            };

            result.push_back(game_element);
        }

        result
    }
}

fn main() {
    println!("{:?}", Game::new(MAP_1, GAME_ELEMENTS_1));
}
