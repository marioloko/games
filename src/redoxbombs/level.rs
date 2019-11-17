use game_element::parser::GameElementsLoader;
use game_element::{Bomb, Enemy, Player, Stairs, Fire};
use maze::Maze;

const MAP_1: &'static [u8] = include_bytes!("assets/levels/1/map.txt");
const GAME_ELEMENTS_1: &'static str = include_str!("assets/levels/1/game_elements.txt");

const MAP_2: &'static [u8] = include_bytes!("assets/levels/2/map.txt");
const GAME_ELEMENTS_2: &'static str = include_str!("assets/levels/2/game_elements.txt");

/// `RAW_LEVELS` contains the map and game element information to generate
/// the different levels.
const RAW_LEVELS: &'static [RawLevel] = &[RawLevel {
    map: MAP_1,
    game_elements: GAME_ELEMENTS_1,
}, RawLevel {
    map: MAP_2,
    game_elements: GAME_ELEMENTS_2,
}];

/// A `RawLevel` stores the map and game elements information needed
/// to generate a level.
struct RawLevel<'a> {
    /// The `ASCII` representation of the map.
    map: &'a [u8],

    /// The text representation of the game elements to be used by a parser.
    game_elements: &'a str,
}

/// A `Level` stores information about the map to generate in a level,
/// and the game elements of that level.
pub struct Level {
    /// The sequence number of the current level.
    index: usize,

    /// The `Maze` used during this level.
    pub maze: Maze,

    /// The element representing the `Player` during this level.
    pub player: Player,

    /// The set of enemies during this level.
    pub enemies: Vec<Option<Enemy>>,

    /// The stairs to go to the next level.
    pub stairs: Stairs,

    /// Bombs in the `Maze`.
    pub bombs: Vec<Option<Bomb>>,

    /// Fires in the `Maze`.
    pub fires: Vec<Option<Fire>>,
}

impl Level {
    /// Create a `Level` object pointing to index 0 (first level).
    ///
    /// panics:
    /// - If there is no level defined.
    pub fn new() -> Level {
        let index = 0;
        Level::with_index(index).expect("Expected at least one level.")
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
        let stairs = loader.generate_stairs();
        let enemies = loader.generate_enemies().into_iter().map(Some).collect();
        let bombs = loader.generate_bombs().into_iter().map(Some).collect();
        let fires = Vec::new();

        let level = Level {
            index,
            maze,
            player,
            enemies,
            stairs,
            bombs,
            fires,
        };

        Some(level)
    }

    /// Return the current level.
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Add a bomb to the level and return the bomb id.
    pub fn add_bomb(&mut self, bomb: Bomb) -> usize {
        // Add the bomb to the vector.
        let bomb = Some(bomb);
        self.bombs.push(bomb);

        // The bomb id is its position in the vector.
        let id = self.bombs.len() - 1;
        id
    }

    /// Add a fire to the level and return the fire id.
    pub fn add_fire(&mut self, fire: Fire) -> usize {
        // Add the bomb to the vector.
        let fire = Some(fire);
        self.fires.push(fire);

        // The bomb id is its position in the vector.
        let id = self.fires.len() - 1;
        id
    }
}
