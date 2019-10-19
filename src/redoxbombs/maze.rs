use std::fmt;

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    BreakableWall,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let render_char = match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::BreakableWall => '=',
        };

        write!(f, "{}", render_char)
    }
}

#[derive(Debug)]
pub struct Maze {
    tiles: Vec<Tile>,
    width: usize,
}

impl Maze {
    pub fn empty() -> Maze {
        Maze {
            tiles: Vec::with_capacity(0),
            width: 0,
        }
    }

    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        let tile = &self.tiles[x + self.width * y];

        match tile {
            Tile::Empty => false,
            Tile::Wall | Tile::BreakableWall => true,
        }
    }
}

impl From<&[u8]> for Maze {
    fn from(map: &[u8]) -> Self {
        let mut tiles = Vec::with_capacity(map.len());

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
