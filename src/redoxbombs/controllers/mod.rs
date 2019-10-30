mod input;
mod output;
mod time;

// Export `InputController` and `OutputController` under
// controller namespace.
pub use self::input::InputController;
pub use self::output::OutputController;
pub use self::time::TimeController;
