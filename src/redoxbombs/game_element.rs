use events::{Direction, InputEvent, InputEvents, ResultEvent};
use maze::Maze;
use std::collections::VecDeque;
use std::fmt;
use std::ops::Sub;
use rand::{thread_rng, Rng};

pub type GameElementObject<'a> = Box<dyn GameElement + 'a>;
pub type GameElementObjects<'a> = VecDeque<GameElementObject<'a>>;

#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

fn abs_sub<T>(x: T, y: T) -> <T as std::ops::Sub>::Output
    where T: Sub + PartialOrd {
    if x > y {
        x - y
    } else {
        y - x
    }
}

impl Coordinates {
    fn euclidean_distance(&self, other: &Coordinates) -> f64 {
        let dx = abs_sub(self.x, other.x);
        let dy = abs_sub(self.y, other.y);

        let square_sum = dx.pow(2) + dy.pow(2);

        (square_sum as f64).sqrt()
    }

    fn manhattan_distance(&self, other: &Coordinates) -> usize {
        let dx = abs_sub(self.x, other.x);
        let dy = abs_sub(self.y, other.y);

        dx + dy
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

        let mut directions = vec![
            self.position.up(),
            self.position.down(),
            self.position.left(),
            self.position.right(),
            self.position
        ];

        // Unorder them to increase movement randomness
        thread_rng().shuffle(&mut directions);
        

        let next_position = directions
            .into_iter()
            .filter(|dir| !maze.is_blocked(dir.x, dir.y))
            .map(|dir| {
                let dist = dir.manhattan_distance(player.get_position());
                (dir, dist)
            })
            .max_by(|(_, dist1), (_, dist2)| {
                dist2.partial_cmp(dist1).unwrap()
            })
            .map(|(dir, _)| dir)
            .unwrap();
            
        self.position = next_position;

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
