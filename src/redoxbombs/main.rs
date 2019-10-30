extern crate rand;
extern crate termion;

mod controllers;
mod events;
mod game_element;
mod level;
mod maze;

use controllers::InputController;
use controllers::OutputController;
use controllers::TimeController;
use events::{InputEvent, InputEvents, ResultEvent, ResultEvents};
use level::Level;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

/// A `Game` contains information about how to handle the input, output,
/// the events and the game state.
struct Game<R: Read, W: Write> {
    input_controller: InputController<R>,
    output_controller: OutputController<W>,
    level: Level,
    input_events: InputEvents,
    result_events: ResultEvents,
    time_controller: TimeController,
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
        let input_events = { 0..enemies_count }
            .map(|id| InputEvent::EnemyRelease { id })
            .collect();

        // At the beginning there is no input event.
        let result_events = ResultEvents::new();

        // Init the time controller.
        let time_controller = TimeController::new();

        Game {
            input_controller,
            output_controller,
            level,
            input_events,
            result_events,
            time_controller,
        }
    }

    /// Start the main game loop.
    fn start(&mut self) {
        self.render();

        'main: loop {
            self.input_controller.read_event(&mut self.input_events);

            while let Some(input_event) = self.time_controller.pop_event() {
                self.input_events.push_back(input_event);
            }

            // Read all the input events in the queue and push the results to the
            // result events queue.
            while let Some(input_event) = self.input_events.pop_front() {
                let result_event = match input_event {
                    InputEvent::GameQuit => break 'main,
                    InputEvent::PlayerMove(_) => {
                        self.level.player.take_turn(&self.level.maze, input_event)
                    }
                    InputEvent::EnemyRelease { id } => self
                        .level
                        .enemies
                        .get_mut(id)
                        .unwrap()
                        .take_turn(&self.level.player, &self.level.maze, input_event),
                };

                self.result_events.push_back(result_event);
            }

            // Handle all the result events.
            while let Some(result_event) = self.result_events.pop_front() {
                match result_event {
                    ResultEvent::NextLevel => {
                        let next_level = self.level.next().expect("There is no next level");

                        self.level = next_level;
                    }
                    ResultEvent::EnemyBlock { id } => {
                        let input_event = InputEvent::EnemyRelease { id };
                        self.time_controller.schedule_event_in(500, input_event);
                    }
                    ResultEvent::PlayerDied => break 'main,
                    ResultEvent::EnemyDied { id } => unimplemented!(),
                    ResultEvent::DoNothing => continue,
                }
            }

            thread::sleep(Duration::from_millis(20));
            self.render();
        }
    }

    /// Render the maze and the game elements on the screen.
    fn render(&mut self) {
        self.output_controller.clear();

        // Draw the maze.
        self.output_controller.draw_maze(&self.level.maze);

        // Draw the player.
        self.output_controller.draw_game_element(&self.level.player);

        // Draw the enemies.
        self.output_controller
            .draw_game_elements(&self.level.enemies);

        // Draw the stairs.
        self.output_controller.draw_game_element(&self.level.stairs);

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
    let stdin = termion::async_stdin();
    let mut stdout = io::stdout();

    let mut game = Game::new(stdin, stdout);
    game.start();
}
