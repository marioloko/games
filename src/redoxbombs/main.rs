extern crate termion;

mod game_element;
mod maze;

use termion::{clear, cursor};
use termion::raw::{RawTerminal, IntoRawMode};
use game_element::GameElementObjects;
use maze::Maze;
use std::io::{self, Read, Write};
use std::fmt;

const MAP_1: &'static [u8] = include_bytes!("map1.txt");
const GAME_ELEMENTS_1: &'static str = include_str!("game_elements1.txt");

const LEVELS: &'static [Level] = &[Level {
    map: MAP_1,
    game_elements: GAME_ELEMENTS_1,
}];

struct Level<'a> {
    map: &'a [u8],
    game_elements: &'a str,
}

#[derive(Debug)]
struct Game<'a, R: Read, W: Write> {
    stdin: R,
    stdout: W,
    maze: Maze,
    game_elements: GameElementObjects<'a>,
    level: u8,
}


impl<'a, R: Read, W: Write> Game<'a, R, RawTerminal<W>> {
    fn new(stdin: R, stdout: W) -> Game<'a, R, RawTerminal<W>> {
        let level: u8 = 0;
        let level_info = &LEVELS[level as usize];

        let maze = Maze::from(level_info.map);
        let game_elements = load_game_elements(level_info.game_elements);

        let mut stdout = stdout.into_raw_mode()
            .expect("Stdout cannot be converted to Raw Mode");

        Game {
            stdin,
            stdout,
            maze,
            game_elements,
            level,
        }
    }

    fn start(&mut self) {
        self.render();

        loop {
            let len = self.game_elements.len();
            for _ in { 0..len } {
                let mut game_element = self
                    .game_elements
                    .pop_front()
                    .expect("There is no game element in the game.");

                game_element.take_turn(&self.game_elements, &self.maze);

                self.game_elements.push_back(game_element);
            }

            self.render();
        }
    }

    fn render(&mut self) {
        let game = self.to_string();
        write!(self.stdout, "{clear}{cursor}{game}{hide}", 
                clear = clear::All,
                cursor = cursor::Goto(1, 1),
                game = game,
                hide = cursor::Hide)
            .expect("Cannot render game.");

        self.stdout.flush().unwrap();
    }
}

impl<'a, R: Read, W: Write> fmt::Display for Game<'a, R, RawTerminal<W>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        let map = map.into_iter()
            .map(|line| line.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n\r");

        write!(f, "{}", map)
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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut game = Game::new(stdin, stdout);
    game.start();
}
