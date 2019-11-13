extern crate rand;
extern crate termion;

mod controllers;
mod events;
mod game_element;
mod level;
mod maze;

use controllers::{InputController, OutputController, TimeController};
use events::{GameEvent, InputEvent, ResultEvent};
use level::Level;
use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

/// Milliseconds to sleep when paused to reduce the CPU usage
/// due to busy waiting.
const PAUSE_SLEEP_MILLIS: u64 = 300;
const REFRESH_TIME: u64 = 50;

/// The `GameMode` defines the state of the game.
enum GameMode {
    /// If the game is not blocked and reading for user inputs.
    /// The game is in the main loop.
    Running,

    /// If the game is blocked but inside the main loop.
    Paused,

    /// If the game is exiting the main loop.
    Ended,
}

/// A `Game` contains information about how to handle the input, output,
/// the events and the game state.
struct Game<R: Read, W: Write> {
    /// The component charged with reading the input and converting
    /// it to the appropriated `InputEvents`.
    input_controller: InputController<R>,

    /// The component charged with writing to the output.
    output_controller: OutputController<W>,

    /// The representation of the current level. It holds
    /// the `Maze` and the different `GameElements`.
    level: Level,

    /// A queue holding every input event entered by the user.
    input_events: VecDeque<InputEvent>,

    /// A queue holding every event generated by the game.
    game_events: VecDeque<GameEvent>,

    /// A queue holding every result event.
    result_events: VecDeque<ResultEvent>,

    /// The controller charged with scheduled events.
    time_controller: TimeController,

    /// The meta status of the game.
    game_mode: GameMode,

    /// Whether is necessary or not to render all elements.
    full_render_needed: bool
}

impl<R: Read, W: Write> Game<R, W> {
    /// Initializes a new game.
    fn new(stdin: R, stdout: W) -> Game<R, W> {
        let input_controller = InputController::new(stdin);
        let output_controller = OutputController::new(stdout);
        let level = Level::new();

        // At the beginning there is no input event.
        let input_events = VecDeque::new();

        // Initialize the basic game events to allow game elements to take
        // first turn.
        let game_events = generate_init_game_events(&level);

        // At the beginning there is no result event.
        let result_events = VecDeque::new();

        // Init the time controller.
        let time_controller = TimeController::new();

        // The game is running by default.
        let game_mode = GameMode::Running;

        // At the beggining a full render is necessary.
        let full_render_needed = false;

        Game {
            input_controller,
            output_controller,
            level,
            input_events,
            game_events,
            result_events,
            time_controller,
            game_mode,
            full_render_needed,
        }
    }

    /// Start the main game loop.
    fn start(&mut self) {
        // Render the game elements.
        self.full_render();

        while let GameMode::Running = self.game_mode {
            // Read one input event.
            if let Some(input_event) = self.input_controller.next_event() {
                self.input_events.push_back(input_event);
            }

            // Handle every scheduled event.
            while let Some(game_event) = self.time_controller.pop_event() {
                self.game_events.push_back(game_event);
            }

            // Handle every input event in the queue and generate the
            // corresponding result event.
            while let Some(input_event) = self.input_events.pop_front() {
                let result_event = self.handle_input_event(input_event);
                self.result_events.push_back(result_event);
            }

            // Handle every game event in the queue and generate the
            // corresponding result event.
            while let Some(game_event) = self.game_events.pop_front() {
                let result_event = self.handle_game_event(game_event);
                self.result_events.push_back(result_event);
            }

            // Handle all the result events.
            while let Some(result_event) = self.result_events.pop_front() {
                self.handle_result_event(result_event);
            }

            // Check whether the game has been paused and handle it.
            self.handle_pause();

            // Avoid inmediate redrawing of the maze.
            thread::sleep(Duration::from_millis(REFRESH_TIME));

            // If full render is needed then render the map and all its
            // elements.
            if self.full_render_needed {
                self.full_render();
                self.full_render_needed = false;
            } else {
                self.output_controller.render();
            }
        }
    }

    /// It consumes an `InputEvent` processes it and generate the correspondant
    /// result event. This function can modify the state of the game elements
    /// of the current level.
    fn handle_input_event(&mut self, input_event: InputEvent) -> ResultEvent {
        match input_event {
            InputEvent::PlayerMove(_) => {
                // Clear player before moving.
                self.output_controller.clear_game_element(&self.level.player);

                // Move player.
                let event = self.level.player.take_turn(&self.level.maze, input_event);

                // Draw player after moving.
                self.output_controller.draw_game_element(&self.level.player);
                event
            }
            InputEvent::PlayerCreateBomb => self.level.player.take_turn(&self.level.maze, input_event),
            InputEvent::GameQuit => ResultEvent::GameExit,
            InputEvent::GamePause => ResultEvent::GamePause,
        }
    }

    /// It consumes an `GameEvent` processes it and generate the correspondant
    /// result event. This function can modify the state of the game elements
    /// of the current level.
    fn handle_game_event(&mut self, game_event: GameEvent) -> ResultEvent {
        match game_event {
            GameEvent::EnemyRelease { id } => {
                match self.level.enemies.get_mut(id).unwrap_or(&mut None) {
                    Some(enemy) => {
                        // Clear enemy before moving.
                        self.output_controller.clear_game_element(enemy);

                        // Move enemy.
                        let event = enemy.take_turn(&self.level.player, &self.level.maze, game_event);

                        // Redraw enemy after moving.
                        self.output_controller.draw_game_element(enemy);

                        event
                    }
                    _ => ResultEvent::DoNothing,
                }
            }
            GameEvent::EnemyCheckCollision { id } => {
                match self.level.enemies.get_mut(id).unwrap_or(&mut None) {
                    Some(enemy) => enemy.take_turn(&self.level.player, &self.level.maze, game_event),
                    _ => ResultEvent::DoNothing,
                }
            }
            GameEvent::StairsRelease => {
                self.level
                    .stairs
                    .take_turn(&self.level.player, &self.level.maze, game_event)
            }
            GameEvent::BombExplode { id } | GameEvent::BombInit { id } => {
                match self.level.bombs.get(id).unwrap() {
                    Some(bomb) => bomb.take_turn(game_event),
                    _ => ResultEvent::DoNothing,
                }
            }
        }
    }

    /// It consumes a `ResultEvent` and perform the correspondant processing
    /// for that event.
    fn handle_result_event(&mut self, result_event: ResultEvent) {
        match result_event {
            ResultEvent::EnemyBlock { id } => {
                let game_event = GameEvent::EnemyRelease { id };
                self.time_controller.schedule_event_in(500, game_event);
            }
            ResultEvent::NextLevel => {
                let next_level = self.level.next().expect("There is no next level.");

                self.level = next_level;

                // Draw again the map.
                self.full_render_needed = true;
            }
            ResultEvent::PlayerDied | ResultEvent::GameExit => {
                self.game_mode = GameMode::Ended;
            }
            ResultEvent::GamePause => {
                self.game_mode = GameMode::Paused;
            }
            ResultEvent::StairsBlock => {
                self.game_events.push_back(GameEvent::StairsRelease);
            }
            ResultEvent::EnemyCheckCollision { id } => {
                self.game_events
                    .push_back(GameEvent::EnemyCheckCollision { id });
            }
            ResultEvent::BombInit { id } => {
                // Create a GameEvent to explode and schedule it.
                let game_event = GameEvent::BombExplode { id };
                self.time_controller.schedule_event_in(3_000, game_event);

                // Draw initialized bomb.
                if let Some(bomb) = self.level.bombs.get(id).unwrap_or(&None) {
                    self.output_controller.draw_game_element(bomb);
                }
            }
            ResultEvent::BombCreated { bomb } => {
                // Add bomb to the level and get its id.
                let id = self.level.add_bomb(bomb);

                // Create a GameEvent to explode and schedule it.
                let game_event = GameEvent::BombExplode { id };
                self.time_controller.schedule_event_in(3_000, game_event);

                // Draw created bomb.
                if let Some(bomb) = self.level.bombs.get(id).unwrap_or(&None) {
                    self.output_controller.draw_game_element(bomb);
                }
            }
            ResultEvent::BombExplode { id } => {
                // Discard bomb at exploding time.
                if let Some(bomb) = self.level.bombs[id].take() {
                    // Clear the bomb from the screen.
                    self.output_controller.clear_game_element(&bomb);
                }
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

    /// Render the maze and all the game elements on the screen.
    fn full_render(&mut self) {
        self.output_controller.clear();

        // Draw the maze.
        self.output_controller.draw_maze(&self.level.maze);

        // Draw the stairs.
        self.output_controller.draw_game_element(&self.level.stairs);

        // Draw the bombs.
        self.output_controller
            .draw_optional_game_elements(&self.level.bombs);

        // Draw the player.
        self.output_controller.draw_game_element(&self.level.player);

        // Draw the enemies.
        self.output_controller
            .draw_optional_game_elements(&self.level.enemies);

        self.output_controller.render();
    }
}

/// Gnerate the initial events to wake up every `GameElement` at least once.
/// Gnerate the initial events to wake up every `GameElement` at least once.
fn generate_init_game_events(level: &Level) -> VecDeque<GameEvent> {
    let mut game_events = VecDeque::new();

    // Generate basic enemy game events.
    for (id, _) in level.enemies.iter().enumerate() {
        game_events.push_back(GameEvent::EnemyRelease { id });
        game_events.push_back(GameEvent::EnemyCheckCollision { id });
    }

    // Generate initialization game events for bombs.
    for (id, _) in level.bombs.iter().enumerate() {
        game_events.push_back(GameEvent::BombInit { id });
    }

    // Generate a release event for stairs to force it to take turn at least
    // once.
    game_events.push_back(GameEvent::StairsRelease);

    game_events
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
