use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};
use std::io::Write;
use maze::Maze;
use game_element::{GameElementObject, GameElementObjects};

pub struct OutputController<W: Write> {
    output: RawTerminal<W>,
}

impl<W: Write> OutputController<W> {
    pub fn new(output: W) -> OutputController<W> {
        let output = output
            .into_raw_mode()
            .expect("OutputController cannot convert its output to raw mode.");

        OutputController { output }
    }

    pub fn render(&mut self) {
        self.output.flush();
    }

    pub fn clear(&mut self) {
        write!(self.output, "{}", clear::All)
            .expect("OutputController cannot clear output");
    }

    pub fn draw_maze(&mut self, maze: &Maze) {
        let maze = maze.to_string().replace("\n", "\n\r");

        write!(
            self.output, 
            "{cursor}{maze}{hide}",
            cursor = cursor::Goto(1, 1),
            maze = maze.to_string(),
            hide = cursor::Hide
        )
        .expect("OutputController cannot draw the map.");
    }

    pub fn draw_game_elements(&mut self, game_elements: &GameElementObjects) {
        for game_element in game_elements {
            self.draw_game_element(game_element);
        }
    }

    fn draw_game_element(&mut self, game_element: &GameElementObject) {
        let position = game_element.get_position();
        let x = 1 + position.x as u16;
        let y = 1 + position.y as u16;

        let representation = game_element.get_representation();

        write!(
            self.output,
            "{cursor}{game_element}",
            cursor = cursor::Goto(x, y),
            game_element = representation
        )
        .unwrap_or_else(|_| {
            panic!(
                "OutputController Cannot draw game element: \
                 {game_element}, at pos: ({x},{y})",
                game_element = representation,
                x = x,
                y = y
            )
        });
    }

}
