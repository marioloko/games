//use std::io::{Read, Write};
mod game_element;
mod maze;

use game_element::GameElementObjects;
use maze::Maze;

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
        game_elements
            .lines()
            .map(|line| {
                let mut it = line.split(' ');

                let name = it.next().expect("Name not found for game element.");

                let x = it
                    .next()
                    .expect("X Coordinate not found for game element.")
                    .parse()
                    .expect("X Coordinate is not a valid integer.");

                let y = it
                    .next()
                    .expect("X Coordinate not found for game element.")
                    .parse()
                    .expect("X Coordinate is not a valid integer.");

                game_element::generate_game_element(name, x, y)
            })
            .collect()
    }

    fn render(&self) -> String {
        let mut map: Vec<Vec<char>> = self
            .maze
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        for game_element in &self.game_elements {
            let position = game_element.get_position();

            map[position.y][position.x] = game_element.get_representation();
        }

        map.into_iter()
            .map(|line| line.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn start(&mut self) {
        println!("{}", self.render());

        loop {
            let len = self.game_elements.len();
            for _ in {0 .. len} {
                let mut game_element = self.game_elements.pop_front()
                    .expect("There is no game element in the game.");

                game_element.take_turn(&self.game_elements);

                self.game_elements.push_back(game_element);
            }

            println!("{}", self.render());
        }
    }
}

fn main() {
    let mut game = Game::new(MAP_1, GAME_ELEMENTS_1);
    game.start();
}
