extern crate termion;
extern crate rand;

mod events;
mod game_element;
mod controllers;
mod maze;
mod parser;
mod level;

use std::collections::VecDeque;
use controllers::InputController;
use controllers::OutputController;
use events::{InputEvent, ResultEvent};
use std::io::{self, Read, Write};
use level::Level;

/// A `Game` contains information about how to handle the input, output,
/// the events and the game state.
struct Game<R: Read, W: Write> {
    input_controller: InputController<R>,
    output_controller: OutputController<W>,
    level: Level,
    input_events: VecDeque<InputEvent>,
    result_events: VecDeque<ResultEvent>,
}

impl<R: Read, W: Write> Game<R, W> {
    /// Initializes a new game.
    fn new(stdin: R, stdout: W) -> Game<R, W> {
        let input_controller = InputController::new(stdin);
        let output_controller = OutputController::new(stdout);
        let level = Level::new();

        // Create a release event for every enemy to allow them to
        // start moving.
        let enemies_count = level.enemies.len();
        let input_events = {0..enemies_count}
            .map(|id| {
                InputEvent::EnemyRelease { id }
            }).collect();

        // At the beginning there is no input event.
        let result_events = VecDeque::new();

        Game {
            input_controller,
            output_controller,
            level,
            input_events,
            result_events,
        }
    }

    /// Start the main game loop.
    fn start(&mut self) {
        self.render();

        'main: loop {
            self.input_controller.read_event(&mut self.input_events);

            // Read all the input events in the queue and push the results to the
            // result events queue.
            while let Some(input_event) = self.input_events.pop_front() {
                let result_event = match input_event {
                    InputEvent::GameQuit => break 'main,
                    InputEvent::PlayerMove(_) => {
                        self.level.player.take_turn(&self.level.maze, input_event)
                    },
                    InputEvent::EnemyRelease { id } => {
                        self.level.enemies
                            .get_mut(id)
                            .unwrap()
                            .take_turn(&self.level.player, &self.level.maze, input_event)
                    },
                };

                self.result_events.push_back(result_event);
            }

            // Handle all the result events.
            while let Some(result_event) = self.result_events.pop_front() {
                match result_event {
                    ResultEvent::NextLevel => {
                        let next_level = self.level
                            .next()
                            .expect("There is no next level");

                        self.level = next_level;
                    },
                    ResultEvent::EnemyBlock { id } => {
                        let input_event = InputEvent::EnemyRelease { id };
                        self.input_events.push_back(input_event);
                    },
                    ResultEvent::PlayerDied => break 'main,
                    ResultEvent::EnemyDied { id } => unimplemented!(),
                    ResultEvent::DoNothing => continue,
                }
            }

            self.render();
        }
    }

    /// Render the maze and the game elements on the screen.
    fn render(&mut self) {
        self.output_controller.clear();

        // Draw the maze.
        self.output_controller
            .draw_maze(&self.level.maze);

        // Draw the player.
        self.output_controller
            .draw_game_element(&self.level.player);

        // Draw the enemies.
        self.output_controller
            .draw_game_elements(&self.level.enemies);

        // Draw the stairs.
        self.output_controller
            .draw_game_element(&self.level.stairs);

        self.output_controller.render();
    }
}

impl<R: Read, W: Write> Drop for Game<R, W> {
    /// Clear the screen game elements on drop.
    fn drop(&mut self) {
        self.output_controller.clear();
        self.output_controller.render();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut game = Game::new(stdin, stdout);
    game.start();
}
