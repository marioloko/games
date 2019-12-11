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

/// Container type to check if any `GameElement` in the container
/// is located at the given `Coordinates`.
///
/// It should only be used with `GameElement` containers.
pub trait AnyGameElementAt {
    /// Check in the container `self` if any `GameElement` is
    /// at coordinates `coord`.
    fn any_game_element_at(&self, coord: &Coordinates) -> bool;
}

impl<T> AnyGameElementAt for &[Option<T>]
where
    T: GameElement,
{
    fn any_game_element_at(&self, coord: &Coordinates) -> bool {
        self.iter().any(|game_element| match game_element {
            Some(game_element) => game_element.get_position() == coord,
            None => false,
        })
    }
}
