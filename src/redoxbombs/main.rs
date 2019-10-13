//use std::io::{Read, Write};
mod game_element;
mod maze;

use game_element::{GameElementObject, GameElementObjects};
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
        game_elements.lines().map(|line| {
            let mut it = line.split(' ');

            let name = it.next().unwrap();
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();

            game_element::generate_game_element(name, x, y)
        }).collect()
    }
}

fn main() {
    println!("{:?}", Game::new(MAP_1, GAME_ELEMENTS_1));
}
