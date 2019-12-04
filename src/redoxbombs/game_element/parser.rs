use game_element::{Fire, Bomb, Enemy, Player, Stairs};
use std::collections::HashMap;

/// A `GameElementsLoader` is a game elements parser. It holds the information
/// needed to parse game elements from string.
pub struct GameElementsLoader<'a> {
    /// Map linking an game_element name with its arguments to create a new
    /// object
    game_elements: HashMap<&'a str, Vec<Vec<&'a str>>>,
}

impl<'a> GameElementsLoader<'a> {
    /// Create a new `GameElementsLoader` from a game elements config string literal.
    ///
    /// input: string literal with the information of the game elements
    ///     in the current level. It has the following format.
    /// ```
    /// Player 1 2
    /// Enemy 7 8
    ///```
    pub fn new(input: &'a str) -> GameElementsLoader<'a> {
        let mut game_elements = HashMap::new();

        // Split the input in several trimmed lines. It discards
        // blank lines and lines starting with "#" comments.
        let lines = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with("#"));

        for line in lines {
            let mut it = line.split_whitespace();

            let name = it.next().expect("Name not found for game element.");

            let arguments = it.collect();

            // Push the arguments to a vector of arguments in which every
            // position represents a different game element of the same type.
            //
            // If it is the first element of a type, then it creates also the
            // vector.
            game_elements
                .entry(name)
                .or_insert(Vec::new())
                .push(arguments);
        }

        GameElementsLoader { game_elements }
    }

    /// Generate a `Player` from the information gathered from the config string
    /// literal. A `Player` definition consists on the name `Player` followed by
    /// its x and y coordinates as integer.
    ///
    /// Ex: `Player 1 2`, where `x = 1` and `y = 2`.
    ///
    /// return: A single Player.
    ///
    /// panics:
    /// - This method panics if no player exists.
    /// - This method panics if `x` coordinate is not and integer.
    /// - This method panics if `y` coordinate is not and integer.
    pub fn generate_player(&self) -> Player {
        let args = self
            .game_elements
            .get(Player::NAME)
            .expect("Expected 1 player on the game elements config, found 0.")
            .get(0)
            .unwrap();

        let x = extract_x_arg(&args, Player::NAME);

        let y = extract_y_arg(&args, Player::NAME);

        Player::new(x, y)
    }

    /// Generate `Enemies` from the information gathered from the config string
    /// literal. A `Enemy`definition consists on the name of the enemy followed by
    /// its x and y coordinates as integer.
    ///
    /// Ex: `Enemy 1 2`, where `x = 1` and `y = 2`.
    ///
    ///
    /// returns: 0 or more `Enemy` objects.
    ///
    /// panics:
    /// - This method panics if any enemy `x` coordinate is not and integer.
    /// - This method panics if any enemy `y` coordinate is not and integer.
    pub fn generate_enemies(&self) -> Vec<Enemy> {
        let enemies = self.game_elements.get(Enemy::NAME);

        let enemies = match enemies {
            None => return Vec::new(),
            Some(enemies) => enemies,
        };

        enemies
            .iter()
            .map(|args| {
                let x = extract_x_arg(args, Enemy::NAME);

                let y = extract_y_arg(args, Enemy::NAME);

                Enemy::new(x, y)
            })
            .collect()
    }

    /// Generate `Vec<Bomb>` from the information gathered from the config string
    /// literal. A `Bomb`definition consists on the name of the bomb followed by
    /// its x and y coordinates as integer.
    ///
    /// Ex: `Bomb 1 2`, where `x = 1` and `y = 2`.
    ///
    ///
    /// returns: 0 or more `Bomb` objects.
    ///
    /// panics:
    /// - This method panics if any bomb `x` coordinate is not and integer.
    /// - This method panics if any bomb `y` coordinate is not and integer.
    pub fn generate_bombs(&self) -> Vec<Bomb> {
        let bombs = self.game_elements.get(Bomb::NAME);

        let bombs = match bombs {
            None => return Vec::new(),
            Some(bombs) => bombs,
        };

        bombs
            .iter()
            .map(|args| {
                let x = extract_x_arg(args, Bomb::NAME);

                let y = extract_y_arg(args, Bomb::NAME);

                Bomb::new(x, y)
            })
            .collect()
    }

    /// Generate `Vec<Fire>` from the information gathered from the config string
    /// literal. A `Fire` definition consists on the name of the fire followed by
    /// its x and y coordinates as integer and the duration also as integer.
    ///
    /// Ex: `Fire 1 2 200`, where `x = 1`,  `y = 2` and `duration = 200`.
    ///
    ///
    /// returns: 0 or more `Fire` objects.
    ///
    /// panics:
    /// - This method panics if any fire `x` coordinate is not and integer.
    /// - This method panics if any fire `y` coordinate is not and integer.
    /// - This method panics if any fire `duration` is not and integer.
    pub fn generate_fires(&self) -> Vec<Fire> {
        let fires = self.game_elements.get(Fire::NAME);

        let fires = match fires {
            None => return Vec::new(),
            Some(fires) => fires,
        };

        fires
            .iter()
            .map(|args| {
                let x = extract_x_arg(args, Fire::NAME);

                let y = extract_y_arg(args, Fire::NAME);

                let duration = extract_duration_arg(args, Fire::NAME) as u64;

                Fire::new(x, y, duration)
            })
            .collect()
    }

    /// Generate a `Stairs` object from the information gathered from
    /// the config string literal. A `Stairs` object definition consists
    /// on the name `Stairs` followed by its x and y coordinates as integer.
    ///
    /// Ex: `Stairs 1 2`, where `x = 1` and `y = 2`.
    ///
    /// returns: A single Stairs object.
    ///
    /// panics:
    /// - If no stair exists.
    /// - If `x` coordinate is not and integer.
    /// - If `y` coordinate is not and integer.
    pub fn generate_stairs(&self) -> Stairs {
        let args = self
            .game_elements
            .get(Stairs::NAME)
            .expect("Expected 1 stairs object on the game elements config, found 0.")
            .get(0)
            .unwrap();

        let x = extract_x_arg(args, Stairs::NAME);

        let y = extract_y_arg(args, Stairs::NAME);

        Stairs::new(x, y)
    }
}

/// Extract the argument with index `$index` from the argument list `$args`. It also
/// uses the `$game_element_name` and `$argument_name` to provide debuggin info.
fn extract_arg(args: &[&str], index: usize, game_element_name: &str, argument_name: &str) -> usize {
    args.get(index)
        .expect(&format!(
            "{argument_name} not found for {game_element_name}.",
            argument_name = argument_name,
            game_element_name = game_element_name,
        ))
        .parse()
        .expect(&format!(
            "{game_element_name} {argument_name} is not a valid integer.",
            argument_name = argument_name,
            game_element_name = game_element_name,
        ))
}

/// Extract the `x` coordinate from the arguments to create the game element,
/// defined by `game_element_name`.
///
/// returns: The `x` coordinate defined in `args`.
///
/// panics:
/// - If `x` coordinate is not and integer.
fn extract_x_arg(args: &[&str], game_element_name: &str) -> usize {
    extract_arg(args, 0, game_element_name, "X Coordinate")
}

/// Extract the `y` coordinate from the arguments to create the game element,
/// defined by `game_element_name`.
///
/// returns: The `y` coordinate defined in `args`.
///
/// panics:
/// - If `y` coordinate is not and integer.
fn extract_y_arg(args: &[&str], game_element_name: &str) -> usize {
    extract_arg(args, 1, game_element_name, "Y Coordinate")
}

/// Extract the `duration` from the arguments to create the game element,
/// defined by `game_element_name`.
///
/// returns: The `duration` coordinate defined in `args`.
///
/// panics:
/// - If `duration` is not and integer.
fn extract_duration_arg(args: &[&str], game_element_name: &str) -> usize {
    extract_arg(args, 2, game_element_name, "Duration")
}
