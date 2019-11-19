use game_element::GameElement;
use maze::Maze;
use std::io::Write;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, style};

/// The `OutputController` writes the game state using
/// a `RawTerminal`.
pub struct OutputController<W: Write> {
    /// A non cannonical writer.
    output: RawTerminal<W>,
}

impl<W: Write> OutputController<W> {
    /// Create a `OutputController` from an object which implements the
    /// Write trait.
    ///
    /// The output is converted to `RawTerminal` to change the TTY to non
    /// cannonical mode. This enables reading from the TTY character by
    /// character, without waiting for new line.
    ///
    /// panics:
    /// - If output cannot be converted to `RawTerminal`.
    pub fn new(output: W) -> OutputController<W> {
        // Convert standard output to raw terminal.
        let output = output
            .into_raw_mode()
            .expect("OutputController cannot convert its output to raw mode.");

        OutputController { output }
    }

    /// Render drawn elements.
    pub fn render(&mut self) {
        self.output.flush();
    }

    /// Remove all the drawn elements.
    ///
    /// panics:
    /// - If it is not possible to clear the screen.
    pub fn clear(&mut self) {
        write!(
            self.output,
            "{clear}{style}{cursor}",
            clear = clear::All,
            style = style::Reset,
            cursor = cursor::Goto(1, 1),
        )
        .expect("OutputController cannot clear output");
    }

    /// Remove a game element.
    ///
    /// panics:
    /// - If it is not possible to clear the screen.
    pub fn clear_game_element(&mut self, game_element: &impl GameElement) {
        let position = game_element.get_position();
        let x = 1 + position.x as u16;
        let y = 1 + position.y as u16;

        write!(self.output, "{cursor} ", cursor = cursor::Goto(x, y),)
            .expect("OutputController cannot clear game element.");
    }

    /// Draw the maze using the output. (But it is not render on
    /// the screen until `render` is called).
    ///
    /// panics:
    /// - If the maze cannot be drawn.
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

    /// Draw every given game element. (But they are not render on
    /// the screen until `render` is called).
    pub fn draw_game_elements(&mut self, game_elements: &[impl GameElement]) {
        for game_element in game_elements {
            self.draw_game_element(game_element);
        }
    }

    /// Draw every optional game element. (But they are not render on
    /// the screen until `render` is called).
    pub fn draw_optional_game_elements(&mut self, game_elements: &[Option<impl GameElement>]) {
        for game_element in game_elements {
            match game_element {
                Some(game_element) => self.draw_game_element(game_element),
                _ => continue,
            }
        }
    }

    /// Draw a game element in the location defined by its coordinates.
    /// (But it is not render on the screen until `render` is called).
    ///
    /// panics:
    /// - If it is not possible to draw the given game element.
    pub fn draw_game_element(&mut self, game_element: &impl GameElement) {
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
