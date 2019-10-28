use events::{Direction, InputEvent, InputEvents};
use std::io::Read;

/// The `InputController` reads the user input and
/// translate it into `InputEvents` associated with
/// a task.
pub struct InputController<R: Read> {
    input: R,
}

impl<R: Read> InputController<R> {
    /// Create an `InputController` from any object which implements
    /// the read trait.
    pub fn new(input: R) -> InputController<R> {
        InputController { input }
    }

    /// Read a keyboard event and convert it to its correspondant
    /// `InputEvent`.
    pub fn read_event(&mut self, events: &mut InputEvents) {
        let mut b = [0];

        self.input.read(&mut b).unwrap();

        match b[0] {
            b'h' => events.push_back(InputEvent::PlayerMove(Direction::Left)),
            b'j' => events.push_back(InputEvent::PlayerMove(Direction::Down)),
            b'k' => events.push_back(InputEvent::PlayerMove(Direction::Up)),
            b'l' => events.push_back(InputEvent::PlayerMove(Direction::Right)),
            b'q' => events.push_back(InputEvent::GameQuit),
            _ => (),
        }
    }
}
