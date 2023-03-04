#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum Player {
    Red,
    Blue,
}
impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum Owner {
    None,
    Owned(Player),
}
#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn next(&self, direction: &Direction, dimensions: &Dimensions) -> Option<Position> {
        let (x, y) = (self.x, self.y);
        match direction {
            Direction::Up => {
                if y < dimensions.y - 1 {
                    return Some(Position { x, y: y + 1 });
                }
            }
            Direction::UpRight => {
                if x < dimensions.x - 1 && y < dimensions.y - 1 {
                    return Some(Position { x: x + 1, y: y + 1 });
                }
            }
            Direction::Right => {
                if x < dimensions.x - 1 {
                    return Some(Position { x: x + 1, y });
                }
            }
            Direction::DownRight => {
                if x < dimensions.x - 1 && y > 0 {
                    return Some(Position { x: x + 1, y: y - 1 });
                }
            }
            Direction::Down => {
                if y > 0 {
                    return Some(Position { x, y: y - 1 });
                }
            }
            Direction::DownLeft => {
                if x > 0 && y > 0 {
                    return Some(Position { x: x - 1, y: y - 1 });
                }
            }
            Direction::Left => {
                if x > 0 {
                    return Some(Position { x: x - 1, y });
                }
            }
            Direction::UpLeft => {
                if x > 0 && y < dimensions.y - 1 {
                    return Some(Position { x: x - 1, y: y + 1 });
                }
            }
        };
        None
    }
}
#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]

pub struct Dimensions {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub static DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct Line {
    curr: Option<Position>,
    dimensions: Dimensions,
    direction: Direction,
}

impl Iterator for Line {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if let Some(next) = self.curr {
            self.curr = next.next(&self.direction, &self.dimensions);
            Some(next)
        } else {
            None
        }
    }
}

impl Line {
    pub fn new(direction: &Direction, start: Position, dimensions: &Dimensions) -> Line {
        Line {
            direction: *direction,
            curr: Some(start),
            dimensions: *dimensions,
        }
    }
}
