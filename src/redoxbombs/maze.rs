use std::fmt;

#[derive(Debug)]
struct Wall {
    breakable: bool
}

impl Wall {
    fn new(breakable: bool) -> Self {
        Self { breakable }
    }
}

#[derive(Debug)]
pub struct Maze {
//    players: Vec<Player>,
//    enemies: Vec<Box<dyn Enemy>>,
    tiles: Vec<Option<Wall>>,
    width: usize,
//    map: &'static [u8],
} 

impl From<&[u8]> for Maze {
    fn from(map: &[u8]) -> Self {
        let mut tiles = Vec::with_capacity(map.len());

        let width = map.iter().take_while(|&b| *b != b'\n').count();

        for &tile in map {
            match tile {
                b'\n' => (),
                b'#'  => tiles.push(Some(Wall::new(false))),
                b'='  => tiles.push(Some(Wall::new(true))), 
                _ => tiles.push(None),
            }
        }

        Self {
            tiles,
            width,
        }        
    }
}
