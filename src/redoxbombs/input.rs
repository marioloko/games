use events::{Direction, InputEvent, InputEvents};
use std::io::Read;

pub struct InputController<R: Read> {
    input: R,
}

impl<R: Read> InputController<R> {
    pub fn new(input: R) -> InputController<R> {
        InputController { input }
    }

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
