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

const PAUSE_SLEEP_MILLIS: u64 = 300;

/// The `GameMode` defines the state of the game.
///
/// The possible values are:
/// - Running: If the game is not blocked and reading
/// for user inputs. The game is in the main loop.
/// - Paused: If the game is blocked but inside the
/// main loop.
/// - Ended: If the game is exiting the main loop.
enum GameMode {
    Running,
    Paused,
    Ended,
}

/// A `Game` contains information about how to handle the input, output,
/// the events and the game state.
struct Game<R: Read, W: Write> {
    input_controller: InputController<R>,
    output_controller: OutputController<W>,
    level: Level,
    input_events: InputEvents,
    result_events: ResultEvents,
    time_controller: TimeController,
    game_mode: GameMode,
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

        // The game is running by default.
        let game_mode = GameMode::Running;

        Game {
            input_controller,
            output_controller,
            level,
            input_events,
            result_events,
            time_controller,
            game_mode,
        }
    }

    /// Start the main game loop.
    fn start(&mut self) {
        self.render();

        while let GameMode::Running = self.game_mode {
            // Read one input event.
            if let Some(input_event) = self.input_controller.next_event() {
                self.input_events.push_back(input_event);
            }

            // Handle every scheduled event.
            while let Some(input_event) = self.time_controller.pop_event() {
                self.input_events.push_back(input_event);
            }

            // Handle every input event in the queue and generate the
            // corresponding result event.
            while let Some(input_event) = self.input_events.pop_front() {
                let result_event = self.handle_input_event(input_event);
                self.result_events.push_back(result_event);
            }

            // Handle all the result events.
            while let Some(result_event) = self.result_events.pop_front() {
                self.handle_result_event(result_event);
            }

            // Check whether the game has been paused and handle it.
            self.handle_pause();

            // Avoid inmediate redrawing of the maze.
            thread::sleep(Duration::from_millis(20));
            self.render();
        }
    }

    /// It consumes an `InputEvent` processes it and generate the correspondant
    /// result event. This function can modify the state of the game elements
    /// of the current level.
    fn handle_input_event(&mut self, input_event: InputEvent) -> ResultEvent {
        match input_event {
            InputEvent::PlayerMove(_) => self.level.player.take_turn(&self.level.maze, input_event),
            InputEvent::EnemyRelease { id } => self.level.enemies.get_mut(id).unwrap().take_turn(
                &self.level.player,
                &self.level.maze,
                input_event,
            ),
            InputEvent::GameQuit => ResultEvent::GameExit,
            InputEvent::GamePause => ResultEvent::GamePause,
        }
    }

    /// It consumes a `ResultEvent` and perform the correspondant processing
    /// for that event.
    fn handle_result_event(&mut self, result_event: ResultEvent) {
        match result_event {
            ResultEvent::EnemyBlock { id } => {
                let input_event = InputEvent::EnemyRelease { id };
                self.time_controller.schedule_event_in(500, input_event);
            }
            ResultEvent::NextLevel => {
                let next_level = self.level.next().expect("There is no next level.");

                self.level = next_level;
            }
            ResultEvent::PlayerDied | ResultEvent::GameExit => {
                self.game_mode = GameMode::Ended;
            }
            ResultEvent::GamePause => {
                self.game_mode = GameMode::Paused;
            }
            ResultEvent::EnemyDied { id } => unimplemented!(),
            ResultEvent::DoNothing => (),
        }
    }

    /// If the game is paused then read `InputEvent`s until a
    /// `GamePause` event is received to continue the game.
    fn handle_pause(&mut self) {
        while let GameMode::Paused = self.game_mode {
            // Discard every InputEvent different from GamePause.
            // If GamePause then leave pause state.
            while let Some(input_event) = self.input_controller.next_event() {
                match input_event {
                    InputEvent::GamePause => {
                        self.game_mode = GameMode::Running;
                    }
                    _ => (),
                }
            }

            // If game continues paused then sleep the thread to avoid wasting
            // computer resources in useless computation.
            if let GameMode::Paused = self.game_mode {
                thread::sleep(Duration::from_millis(PAUSE_SLEEP_MILLIS));
            }
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
    let stdout = io::stdout();

    let mut game = Game::new(stdin, stdout);
    game.start();
}
