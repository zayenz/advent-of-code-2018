type Scalar = i16;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: Scalar,
    y: Scalar,
}

impl Position {
    fn new(x: Scalar, y: Scalar) -> Position {
        Position { x, y }
    }

    fn step(&self, direction: Direction) -> Position {
        let (x, y) = match direction {
            North => (self.x, self.y - 1),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
            East => (self.x + 1, self.y),
        };
        Position { x, y }
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Turn {
    Left,
    Right,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (*self, turn) {
            (North, Left) => West,
            (North, Right) => East,
            (South, Left) => East,
            (South, Right) => West,
            (East, Left) => North,
            (East, Right) => South,
            (West, Left) => South,
            (West, Right) => North,
        }
    }
}
