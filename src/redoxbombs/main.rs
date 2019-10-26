extern crate termion;
extern crate rand;

mod events;
mod game_element;
mod input;
mod maze;
mod output;

use events::{InputEvent, InputEvents, ResultEvent};
use game_element::GameElementObjects;
use input::InputController;
use maze::Maze;
use output::OutputController;
use std::io::{self, Read, Write};

const MAP_1: &'static [u8] = include_bytes!("map1.txt");
const GAME_ELEMENTS_1: &'static str = include_str!("game_elements1.txt");

const LEVELS: &'static [Level] = &[Level {
    map: MAP_1,
    game_elements: GAME_ELEMENTS_1,
}];

/// A `Level` stores information about the map to generate in a level,
/// and the game elements of that level.
struct Level<'a> {
    map: &'a [u8],
    game_elements: &'a str,
}

/// A `Game` contains information about how to handle the input, output,
/// the events and the game state.
struct Game<'a, R: Read, W: Write> {
    input_controller: InputController<R>,
    output_controller: OutputController<W>,
    maze: Maze,
    game_elements: GameElementObjects<'a>,
    events: InputEvents,
    level: u8,
}

impl<'a, R: Read, W: Write> Game<'a, R, W> {
    /// Initializes a new game.
    fn new(stdin: R, stdout: W) -> Game<'a, R, W> {
        let level: u8 = 0;
        let level_info = &LEVELS[level as usize];

        let maze = Maze::from(level_info.map);
        let game_elements = load_game_elements(level_info.game_elements);

        let input_controller = InputController::new(stdin);
        let output_controller = OutputController::new(stdout);

        Game {
            input_controller,
            output_controller,
            maze,
            game_elements,
            events: InputEvents::new(),
            level,
        }
    }

    /// Start the main game loop.
    fn start(&mut self) {
        self.render();

        loop {
            self.input_controller.read_event(&mut self.events);

            if let Some(event) = self.events.front() {
                if event.is_quit_event() {
                    break
                }
            }

            let len = self.game_elements.len();
            for _ in { 0..len } {
                let mut game_element = self
                    .game_elements
                    .pop_front()
                    .expect("There is no game element in the game.");

                let event =
                    game_element.take_turn(&self.game_elements, &self.maze, &mut self.events);

                self.game_elements.push_back(game_element);
            }

            self.render();
        }
    }

    /// Render the maze and the game elements on the screen.
    fn render(&mut self) {
        self.output_controller.clear();
        self.output_controller.draw_maze(&self.maze);
        self.output_controller
            .draw_game_elements(&self.game_elements);
        self.output_controller.render();
    }
}

impl<'a, R: Read, W: Write> Drop for Game<'a, R, W> {
    /// Clear the screen game elements on drop.
    fn drop(&mut self) {
        self.output_controller.clear();
        self.output_controller.render();
    }
}

/// Load game element objects from a string literal representing them.
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
