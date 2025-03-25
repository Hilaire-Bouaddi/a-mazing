#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub struct Cell {
    position: Position,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        Self {
            position: Position { x: x, y: y },
        }
    }

    pub fn from_tuple(t: (usize, usize)) -> Cell {
        Self {
            position: Position { x: t.0, y: t.1 },
        }
    }

    pub fn get_x(&self) -> usize {
        self.position.x
    }

    pub fn get_y(&self) -> usize {
        self.position.y
    }

    pub fn get_position(&self) -> Position {
        self.position
    }
}
