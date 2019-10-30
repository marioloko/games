use game_element::Coordinates;
use std::fmt;

/// The `GameElement` trait represents any element which
/// is plazed in a given `Maze` position and has a
/// character representation.
pub trait GameElement: fmt::Debug {
    /// Returns the positon of the game element.
    fn get_position(&self) -> &Coordinates;

    /// Returns the representation of the game element.
    fn get_representation(&self) -> char;
}
