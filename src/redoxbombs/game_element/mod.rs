/// A `GameElement` parser.
pub mod parser;

mod bomb;
mod coordinates;
mod enemy;
mod fire;
mod game_element;
mod player;
mod stairs;

/// The `Coordiantes` for representing a `GameElement` position.
pub use self::coordinates::Coordinates;

/// The trait for representing a `GameElement`.
pub use self::game_element::{AnyGameElementAt, GameElement};

/// The definition of a `Player`.
pub use self::player::Player;

/// The definition of a `Enemy`.
pub use self::enemy::Enemy;

/// The definition of `Stairs`.
pub use self::stairs::Stairs;

/// The definition of a `Bomb`.
pub use self::bomb::Bomb;

/// The definition of `Fire`.
pub use self::fire::Fire;
