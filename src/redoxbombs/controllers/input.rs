use events::{Direction, InputEvent};
use std::io::Read;
use termion::event::Key;
use termion::input::{Keys, TermRead};

/// The `InputController` reads the user input and
/// translate it into `InputEvent` associated with
/// a task.
pub struct InputController<R: Read> {
    /// Keyboard pressed keys iterator.
    input: Keys<R>,
}

impl<R: Read> InputController<R> {
    /// Create an `InputController` from any object which implements
    /// the read trait.
    pub fn new(input: R) -> InputController<R> {
        InputController {
            input: input.keys(),
        }
    }

    /// Read a keyboard event and convert it to its correspondant
    /// `InputEvent`.
    ///
    /// return:
    /// - PlayerMove(dir) if the user pressed any movement key.
    /// - GamePause if the user pressed the 'Esc' key.
    /// - GameQuit if the user pressed 'q' to exit the game.
    ///
    /// panics:
    /// - If no character was read from stdin.
    pub fn next_event(&mut self) -> Option<InputEvent> {
        let key = match self.input.next() {
            None => return None,
            Some(key) => key.expect("Error reading key inputs with `InputController`."),
        };

        match key {
            Key::Char('h') | Key::Left => Some(InputEvent::PlayerMove(Direction::Left)),
            Key::Char('j') | Key::Down => Some(InputEvent::PlayerMove(Direction::Down)),
            Key::Char('k') | Key::Up => Some(InputEvent::PlayerMove(Direction::Up)),
            Key::Char('l') | Key::Right => Some(InputEvent::PlayerMove(Direction::Right)),
            Key::Char('q') => Some(InputEvent::GameQuit),
            Key::Esc => Some(InputEvent::GamePause),
            _ => None,
        }
    }
}
