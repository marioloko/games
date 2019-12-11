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

/// Milliseconds to sleep between Game loop to reduce the CPU
/// usage due to busy waiting.
const LOOP_SLEEP_MILLIS: u64 = 50;

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

    /// Whether or not the game state has been updated. It is
    /// useful in order to check when to render.
    updated: bool,
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

        // By default the game is not updated.
        let updated = false;

        Game {
            input_controller,
            output_controller,
            level,
            input_events,
            game_events,
            result_events,
            time_controller,
            game_mode,
            updated,
        }
    }

    /// Start the main game loop.
    fn start(&mut self) {
        // Render the game elements.
        self.render();

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
            // corresponding result events.
            while let Some(input_event) = self.input_events.pop_front() {
                self.handle_input_event(input_event);
            }

            // Handle every game event in the queue and generate the
            // corresponding result events.
            while let Some(game_event) = self.game_events.pop_front() {
                self.handle_game_event(game_event);
            }

            // Handle all the result events.
            while let Some(result_event) = self.result_events.pop_front() {
                self.handle_result_event(result_event);
            }

            // Check whether the game has been paused and handle it.
            self.handle_pause();

            // Avoid inmediate redrawing of the maze.
            thread::sleep(Duration::from_millis(LOOP_SLEEP_MILLIS));

            // If the game has been updated then render the game again.
            if self.updated {
                self.render();
                self.updated = false;
            }
        }
    }

    /// It consumes an `InputEvent` processes it and generate the correspondant
    /// result events. This function can modify the state of the game elements
    /// of the current level.
    fn handle_input_event(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::PlayerMove(_) | InputEvent::PlayerCreateBomb => {
                // Forward event to the player.
                self.handle_player_input_event(input_event);
            }
            InputEvent::GameQuit => {
                // Exit Game.
                let result = ResultEvent::GameExit;
                self.result_events.push_back(result);
            }
            InputEvent::GamePause => {
                // Pause the Game.
                let result = ResultEvent::GamePause;
                self.result_events.push_back(result);
            }
        }
    }

    /// It consumes an `GameEvent` processes it and generate the correspondant
    /// result events. This function can modify the state of the game elements
    /// of the current level.
    fn handle_game_event(&mut self, game_event: GameEvent) {
        match game_event {
            GameEvent::PlayerRecoverBomb => {
                // Execute player event.
                self.handle_player_event(game_event);
            }
            GameEvent::EnemyMove { id }
            | GameEvent::EnemyCheckCollision { id }
            | GameEvent::EnemyInit { id } => {
                // Execute enemy event.
                self.handle_enemy_event(game_event, id);
            }
            GameEvent::FireCheckCollision { id }
            | GameEvent::FirePutOut { id }
            | GameEvent::FireInit { id } => {
                // Execute fire event.
                self.handle_fire_event(game_event, id);
            }
            GameEvent::StairsCheckCollision => {
                // Execute stairs event.
                self.handle_stairs_event(game_event);
            }
            GameEvent::BombExplode { id } | GameEvent::BombInit { id } => {
                // Execute bomb event.
                self.handle_bomb_event(game_event, id);
            }
        }
    }

    /// It consumes a `ResultEvent` and perform the correspondant processing
    /// for that event.
    fn handle_result_event(&mut self, result_event: ResultEvent) {
        match result_event {
            ResultEvent::NextLevel => {
                // Load the next level. This will clear the events.
                self.load_next_level();
            }
            ResultEvent::PlayerDied | ResultEvent::GameExit => {
                // Exit game.
                self.game_mode = GameMode::Ended;
            }
            ResultEvent::GamePause => {
                // Change game to paused mode. Lock movement.
                self.game_mode = GameMode::Paused;
            }
            ResultEvent::GameUpdated => {
                // Set the game as updated if any change in the world state.
                self.updated = true;
            }
            ResultEvent::GameScheduleEvent { millis, event } => {
                // Schedule and event after `millis` milliseconds.
                self.time_controller.schedule_event_in(millis, event);
            }
            ResultEvent::GameSetEvent { event } => {
                // Set next game event to execute.
                self.game_events.push_back(event);
            }
            ResultEvent::MazeBreak { x, y } => {
                // Break `Tile` at position `x` and `y`.
                self.level.maze.break_tile(x, y);
            }
            ResultEvent::BombNew { bomb } => {
                // Add bomb to the level and get its id.
                let id = self.level.add_bomb(bomb);

                // Initialize the bomb to explode and check collisions.
                let init_event = GameEvent::BombInit { id };
                self.game_events.push_back(init_event);
            }
            ResultEvent::BombDelete { id } => {
                // Discard bomb at exploding time.
                self.level.bombs[id].take();
            }
            ResultEvent::FireNew { fire } => {
                // Add bomb to the level and get its id.
                let id = self.level.add_fire(fire);

                // Initialize the fire to put out and check collisions.
                let init_event = GameEvent::FireInit { id };
                self.game_events.push_back(init_event);
            }
            ResultEvent::FireDelete { id } => {
                // Discard the fire.
                self.level.fires[id].take();
            }
            ResultEvent::EnemyDelete { id } => {
                // Discard the enemy.
                self.level.enemies[id].take();
            }
        }
    }

    /// If the game is paused then read `InputEvent`s until a
    /// `GamePause` event is received to continue the game.
    fn handle_pause(&mut self) {
        while let GameMode::Paused = self.game_mode {
            // Discard every InputEvent different from GamePause.
            // If GamePause then leave pause state.
            while let Some(input_event) = self.input_controller.next_event() {
                if let InputEvent::GamePause = input_event {
                    self.game_mode = GameMode::Running;
                }
            }

            // If game continues paused then sleep the thread to avoid wasting
            // computer resources in useless computation.
            if let GameMode::Paused = self.game_mode {
                thread::sleep(Duration::from_millis(PAUSE_SLEEP_MILLIS));
            }
        }
    }

    /// It consumes a `InputEvent` sent to the `Player` and forwards to it.
    fn handle_player_input_event(&mut self, event: InputEvent) {
        self.level
            .player
            .update_from_input_event(&self.level.maze, event, &mut self.result_events);
    }

    /// It consumes a `GameEvent` sent to the `Player` and forwards to it.
    fn handle_player_event(&mut self, event: GameEvent) {
        self.level
            .player
            .update(event);
    }

    /// It consumes a `GameEvent` sent to an enemy and forwards to the `Enemy`
    /// with id `id`. The `id` represents the enemy position in the level vector.
    fn handle_enemy_event(&mut self, event: GameEvent, id: usize) {
        if let Some(enemy) = self.level.enemies.get_mut(id).unwrap_or(&mut None) {
            enemy.update(
                &self.level.player,
                &self.level.maze,
                event,
                &mut self.result_events,
            );
        }
    }

    /// It consumes a `GameEvent` sent to the `Stairs` and forwards to them.
    fn handle_stairs_event(&mut self, event: GameEvent) {
        self.level
            .stairs
            .update(&self.level.player, event, &mut self.result_events);
    }

    /// It consumes a `GameEvent` sent to a bomb and forwards to the `Bomb`
    /// with id `id`. The `id` represents the bomb position in the level vector.
    fn handle_bomb_event(&mut self, event: GameEvent, id: usize) {
        if let Some(bomb) = self.level.bombs.get(id).unwrap_or(&None) {
            bomb.update(&self.level.maze, event, &mut self.result_events);
        }
    }

    /// It consumes a `GameEvent` sent to a fire and forwards to the `Fire`
    /// with id `id`. The `id` represents the fire position in the level vector.
    fn handle_fire_event(&mut self, event: GameEvent, id: usize) {
        if let Some(fire) = self.level.fires.get_mut(id).unwrap_or(&mut None) {
            fire.update(
                &self.level.player,
                &self.level.enemies,
                &self.level.maze,
                event,
                &mut self.result_events,
            );
        }
    }

    /// Load the next level. It reset the events to avoid collisions
    /// between different level events.
    fn load_next_level(&mut self) {
        // Update the level to the next one.
        let next_level = self.level.next().expect("There is no next level.");
        self.level = next_level;

        // Remove events for this level.
        self.clear_all_events();

        // Generate input events for the new level.
        self.game_events = generate_init_game_events(&self.level);
    }

    /// Clear all the events in the game.
    fn clear_all_events(&mut self) {
        // Clear the events queues.
        self.input_events.clear();
        self.game_events.clear();
        self.result_events.clear();

        // Clear scheduled events. To avoid reschedule them in the future.
        self.time_controller.clear();
    }

    /// Render the maze and all the game elements on the screen.
    fn render(&mut self) {
        // Clear the whole screen.
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

        // Draw the fires.
        self.output_controller
            .draw_optional_game_elements(&self.level.fires);

        self.output_controller.render();
    }
}

/// Gnerate the initial events to wake up every `GameElement` at least once.
fn generate_init_game_events(level: &Level) -> VecDeque<GameEvent> {
    let mut game_events = VecDeque::new();

    // Generate basic enemy game events.
    for (id, _) in level.enemies.iter().enumerate() {
        game_events.push_back(GameEvent::EnemyInit { id });
    }

    // Generate initialization game events for bombs.
    for (id, _) in level.bombs.iter().enumerate() {
        game_events.push_back(GameEvent::BombInit { id });
    }

    // Generate initialization game events for fire.
    for (id, _) in level.fires.iter().enumerate() {
        game_events.push_back(GameEvent::FireInit { id });
    }

    // Generate an event for the stairs to start checking.
    game_events.push_back(GameEvent::StairsCheckCollision);

    game_events
}

impl<R: Read, W: Write> Drop for Game<R, W> {
    /// Clear the screen game elements and show cursor on drop.
    fn drop(&mut self) {
        self.output_controller.reset();
        self.output_controller.render();
    }
}

fn main() {
    let stdin = termion::async_stdin();
    let stdout = io::stdout();

    let mut game = Game::new(stdin, stdout);
    game.start();
}
