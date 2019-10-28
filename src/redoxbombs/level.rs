use maze::Maze;
use game_element::parser::GameElementsLoader;
use game_element::{Player, Enemy, Stairs};

const MAP_1: &'static [u8] = include_bytes!("assets/levels/1/map.txt");
const GAME_ELEMENTS_1: &'static str = include_str!("assets/levels/1/game_elements.txt");

/// `RAW_LEVELS` contains the map and game element information to generate
/// the different levels.
const RAW_LEVELS: &'static [RawLevel] = &[
    RawLevel {
        map: MAP_1,
        game_elements: GAME_ELEMENTS_1,
    },
];

/// A `RawLevel` stores the map and game elements information needed
/// to generate a level.
struct RawLevel<'a> {
    map: &'a [u8],
    game_elements: &'a str,
}

/// A `Level` stores information about the map to generate in a level,
/// and the game elements of that level.
pub struct Level {
    index: usize,
    pub maze: Maze,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub stairs: Stairs,
}

impl Level {
    /// Create a `Level` object pointing to index 0 (first level).
    ///
    /// panics:
    /// - If there is no level defined.
    pub fn new() -> Level {
        let index = 0;
        Level::with_index(index)
            .expect("Expected at least one level.")
    }

    /// Get the next `Level` according to the index sequence.
    ///
    /// returns:
    /// - None: If there is no next level.
    /// - Some(Level): If there exists a level following the
    ///   current one.
    pub fn next(&self) -> Option<Level> {
        let next_index = self.index + 1;
        Level::with_index(next_index)
    }

    /// Create a `Level` object pointing to a given index, and 
    /// loads its information from the configuration stored in its
    /// correspondant `RawLevel` object.
    ///
    /// returns:
    /// - None: If there is no `RawLevel` with the given index.
    /// - Some(Level): If it is possible to create the level.
    fn with_index(index: usize) -> Option<Level> {
        let raw_level = RAW_LEVELS.get(index)?;

        // Load map defined in the selected RawLevel.
        let maze = Maze::from(raw_level.map);

        // Load the different game elements using the parser.
        let loader = GameElementsLoader::new(raw_level.game_elements);
        let player = loader.generate_player();
        let enemies = loader.generate_enemies();
        let stairs = loader.generate_stairs();

        let level = Level {
            index,
            maze,
            player,
            enemies,
            stairs,
        };

        Some(level)
    }

    /// Return the current level.
    pub fn get_index(&self) -> usize {
        self.index
    }
}
