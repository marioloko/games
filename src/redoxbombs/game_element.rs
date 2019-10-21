use events::{Direction, InputEvent, InputEvents, ResultEvent};
use maze::Maze;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Read};

pub type GameElementObject<'a> = Box<dyn GameElement + 'a>;
pub type GameElementObjects<'a> = VecDeque<GameElementObject<'a>>;

#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    fn real_distance(&self, other: &Coordinates) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        let square_sum = dx.pow(2) + dy.pow(2);

        (square_sum as f64).sqrt()
    }

    fn target_to(&self, other: &Coordinates) -> Coordinates {
        let x = Coordinates::next_to(self.x, other.x);
        let y = Coordinates::next_to(self.y, other.y);

        Coordinates { x, y }
    }

    fn target_x_to(&self, other: &Coordinates) -> Coordinates {
        let x = Coordinates::next_to(self.x, other.x);

        Coordinates { x, y: self.y }
    }

    fn target_y_to(&self, other: &Coordinates) -> Coordinates {
        let y = Coordinates::next_to(self.y, other.y);

        Coordinates { x: self.x, y }
    }

    fn next_to(current: usize, target: usize) -> usize {
        if current > target {
            current - 1
        } else if current < target {
            current + 1
        } else {
            current
        }
    }

    fn up(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Coordinates {
        Coordinates {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Coordinates {
        Coordinates {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameElementType {
    Player,
    Enemy,
    Stairs,
}

pub trait GameElement: fmt::Debug {
    fn get_position(&self) -> &Coordinates;

    fn get_type(&self) -> GameElementType;

    fn get_representation(&self) -> char;

    fn take_turn(
        &mut self,
        elems: &GameElementObjects,
        maze: &Maze,
        events: &mut InputEvents,
    ) -> ResultEvent;
}

pub fn generate_game_element(name: &str, x: usize, y: usize) -> GameElementObject {
    match name {
        Player::NAME => Box::new(Player::new(x, y)),
        MotionlessEnemy::NAME => Box::new(MotionlessEnemy::new(x, y)),
        SlowEnemy::NAME => Box::new(SlowEnemy::new(x, y)),
        Stairs::NAME => Box::new(Stairs::new(x, y)),
        _ => panic!("Unrecognized game element: {}", name),
    }
}

#[derive(Debug)]
struct Player {
    position: Coordinates,
}

impl Player {
    const NAME: &'static str = "Player";
    const TYPE: GameElementType = GameElementType::Player;
    const SPEED: f64 = 1.0;
    const REPRESENTATION: char = '@';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for Player {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(
        &mut self,
        elems: &GameElementObjects,
        maze: &Maze,
        events: &mut InputEvents,
    ) -> ResultEvent {
        {
            let player_events = events.iter().filter(|event| event.is_player_event());

            for player_event in player_events {
                let next = match player_event {
                    InputEvent::PlayerMove(dir) => match dir {
                        Direction::Up => self.position.up(),
                        Direction::Down => self.position.down(),
                        Direction::Left => self.position.left(),
                        Direction::Right => self.position.right(),
                    },
                    _ => self.position,
                };

                if !maze.is_blocked(next.x, next.y) {
                    self.position = next;
                }
            }
        }
        events.retain(|event| !event.is_player_event());

        ResultEvent::DoNothing
    }
}

#[derive(Debug)]
struct MotionlessEnemy {
    position: Coordinates,
}

impl MotionlessEnemy {
    const NAME: &'static str = "MotionlessEnemy";
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.0;
    const REPRESENTATION: char = 'M';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for MotionlessEnemy {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(
        &mut self,
        _elems: &GameElementObjects,
        _maze: &Maze,
        events: &mut InputEvents,
    ) -> ResultEvent {
        ResultEvent::DoNothing
    }
}

#[derive(Debug)]
struct SlowEnemy {
    position: Coordinates,
}

impl SlowEnemy {
    const NAME: &'static str = "SlowEnemy";
    const TYPE: GameElementType = GameElementType::Enemy;
    const SPEED: f64 = 0.25;
    const REPRESENTATION: char = 'S';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for SlowEnemy {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(
        &mut self,
        elems: &GameElementObjects,
        maze: &Maze,
        events: &mut InputEvents,
    ) -> ResultEvent {
        let player = elems
            .iter()
            .filter(|elem| elem.get_type() == Player::TYPE)
            .next()
            .unwrap();

        let next = self.position.target_to(player.get_position());
        if !maze.is_blocked(next.x, next.y) {
            self.position = next;
        }

        ResultEvent::DoNothing
    }
}

#[derive(Debug)]
struct Stairs {
    position: Coordinates,
}

impl Stairs {
    const NAME: &'static str = "Stairs";
    const TYPE: GameElementType = GameElementType::Stairs;
    const REPRESENTATION: char = '%';

    pub fn new(x: usize, y: usize) -> Self {
        let position = Coordinates { x, y };

        Self { position }
    }
}

impl GameElement for Stairs {
    fn get_position(&self) -> &Coordinates {
        &self.position
    }

    fn get_type(&self) -> GameElementType {
        Self::TYPE
    }

    fn get_representation(&self) -> char {
        Self::REPRESENTATION
    }

    fn take_turn(
        &mut self,
        elems: &GameElementObjects,
        maze: &Maze,
        events: &mut InputEvents,
    ) -> ResultEvent {
        ResultEvent::DoNothing
    }
}
