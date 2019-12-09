use std::fmt;

/// Represent a Tile in the map.
///
/// The possible values are:
#[derive(Debug)]
enum Tile {
    /// That position is not blocked and game elements can go through.
    Empty,

    /// That position is blocked then game elements cannot go through.
    Wall,

    /// That position is blocked, then game elements cannot go through,
    /// however if a bomb hits it, then it becomes `Empty`.
    BreakableWall,
}

impl fmt::Display for Tile {
    /// Convert each Tile to its correspondant ASCII representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let render_char = match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::BreakableWall => '=',
        };

        write!(f, "{}", render_char)
    }
}

/// A `Maze` is set of tiles which represent the map state.
#[derive(Debug)]
pub struct Maze {
    /// The state of the different tiles composing the maze.
    tiles: Vec<Tile>,

    /// The width of the `Maze` in tiles.
    width: usize,
}

impl Maze {
    /// Check if certain position of the maze is blocked.
    ///
    /// x: The horizontal coordinate in the maze.
    /// y: The vertical coordinate in the maze.
    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        let tile = self.get_tile(x, y);

        match tile {
            Tile::Empty => false,
            Tile::Wall | Tile::BreakableWall => true,
        }
    }

    /// Check if certain position of the maze represents a blocked breakable `Tile`.
    ///
    /// x: The horizontal coordinate in the maze.
    /// y: The vertical coordinate in the maze.
    pub fn is_blocked_breakable(&self, x: usize, y: usize) -> bool {
        let tile = self.get_tile(x, y);

        match tile {
            Tile::BreakableWall => true,
            Tile::Empty | Tile::Wall => false,
        }
    }

    /// Check if certain position of the maze represents a blocked unbreakable `Tile`.
    ///
    /// x: The horizontal coordinate in the maze.
    /// y: The vertical coordinate in the maze.
    pub fn is_blocked_unbreakable(&self, x: usize, y: usize) -> bool {
        let tile = self.get_tile(x, y);

        match tile {
            Tile::Wall => true,
            Tile::Empty | Tile::BreakableWall => false,
        }
    }

    /// Return a reference to the `Tile` at position `x` and `y`.
    ///
    /// x: The horizontal coordinate in the maze.
    /// y: The vertical coordinate in the maze.
    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x + self.width * y]
    }
}

impl From<&[u8]> for Maze {
    /// Create `Maze` from a byte slice in which every byte represent
    /// the state of a `Tile` by its correspondant ASCII reppresentation.
    fn from(map: &[u8]) -> Self {
        let mut tiles = Vec::with_capacity(map.len());

        // Count the number of characters until the first '\n' as
        // the map width.
        let width = map.iter().take_while(|&b| *b != b'\n').count();

        for &tile in map {
            match tile {
                b'\n' => (),
                b'#' => tiles.push(Tile::Wall),
                b'=' => tiles.push(Tile::BreakableWall),
                _ => tiles.push(Tile::Empty),
            }
        }

        Self { tiles, width }
    }
}

impl fmt::Display for Maze {
    /// Convert a `Maze` to its correspondand ASCII representation.
    /// This representation shows the current state of every `Tile`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if idx % self.width == 0 && idx != 0 {
                write!(f, "\n")?;
            }

            write!(f, "{}", tile)?;
        }

        write!(f, "")
    }
}
