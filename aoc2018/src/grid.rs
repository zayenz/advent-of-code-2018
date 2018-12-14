use hashbrown::HashMap;
use std::cmp::max;
use std::cmp::min;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Index;
use std::ops::IndexMut;
use strum_macros::EnumString;

type Scalar = i32;

pub trait Step<T: Copy>
where
    Self: Copy + Clone,
{
    fn step(&self, direction: T) -> Self;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: Scalar,
    pub y: Scalar,
}

impl Position {
    pub fn new(x: Scalar, y: Scalar) -> Position {
        Position { x, y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Step<Cardinal> for Position {
    fn step(&self, direction: Cardinal) -> Self {
        use crate::grid::Cardinal::*;
        let (x, y) = match direction {
            North => (self.x, self.y - 1),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
            East => (self.x + 1, self.y),
        };
        Position { x, y }
    }
}

impl Step<Direction> for Position {
    fn step(&self, direction: Direction) -> Self {
        use crate::grid::Direction::*;
        let (x, y) = match direction {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Right => (self.x + 1, self.y),
            Left => (self.x - 1, self.y),
        };
        Position { x, y }
    }
}

impl From<(Scalar, Scalar)> for Position {
    fn from(pos: (Scalar, Scalar)) -> Self {
        Position { x: pos.0, y: pos.1 }
    }
}

impl From<&(Scalar, Scalar)> for Position {
    fn from(pos: &(Scalar, Scalar)) -> Self {
        Position { x: pos.0, y: pos.1 }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Position {
            x: pos.0 as Scalar,
            y: pos.1 as Scalar,
        }
    }
}

impl From<&(usize, usize)> for Position {
    fn from(pos: &(usize, usize)) -> Self {
        Position {
            x: pos.0 as Scalar,
            y: pos.1 as Scalar,
        }
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Turn {
    Left,
    Right,
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn turn(self, turn: Turn) -> Cardinal {
        use crate::grid::Cardinal::*;
        use crate::grid::Turn::*;
        match (self, turn) {
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

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn turn(self, turn: Turn) -> Direction {
        use crate::grid::Direction::*;
        match (self, turn) {
            (Up, Turn::Left) => Left,
            (Up, Turn::Right) => Right,
            (Down, Turn::Left) => Right,
            (Down, Turn::Right) => Left,
            (Right, Turn::Left) => Up,
            (Right, Turn::Right) => Down,
            (Left, Turn::Left) => Down,
            (Left, Turn::Right) => Up,
        }
    }
}

impl From<Cardinal> for Direction {
    fn from(cardinal: Cardinal) -> Self {
        use crate::grid::Cardinal::*;
        use crate::grid::Direction::*;
        match cardinal {
            North => Up,
            South => Down,
            East => Right,
            West => Left,
        }
    }
}

impl From<Direction> for Cardinal {
    fn from(direction: Direction) -> Self {
        use crate::grid::Cardinal::*;
        use crate::grid::Direction::*;
        match direction {
            Up => North,
            Down => South,
            Right => East,
            Left => West,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T>
where
    T: Debug + Clone + Eq,
{
    pub values: HashMap<Position, T>,
    pub min_x: Scalar,
    pub min_y: Scalar,
    pub max_x: Scalar,
    pub max_y: Scalar,
}

impl<T> Grid<T>
where
    T: Debug + Clone + Eq,
{
    pub fn new() -> Grid<T> {
        Grid {
            values: HashMap::with_capacity(256),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn filled(
        value: &T,
        min_x: Scalar,
        min_y: Scalar,
        max_x: Scalar,
        max_y: Scalar,
    ) -> Grid<T> {
        let mut grid = Grid::new();
        for x in min_x..max_x {
            for y in min_y..max_y {
                grid.insert((x, y).into(), value.clone());
            }
        }
        grid
    }

    fn update_bounds(&mut self, position: Position) {
        self.min_x = min(self.min_x, position.x);
        self.min_y = min(self.min_y, position.y);
        self.max_x = max(self.max_x, position.x);
        self.max_y = max(self.max_y, position.y);
    }

    pub fn insert(&mut self, position: Position, value: T) {
        self.update_bounds(position);
        self.values.insert(position, value);
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.values.get(&position)
    }
}

impl<T> Default for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn default() -> Self {
        Grid::new()
    }
}

impl<T> Index<Position> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    type Output = T;

    fn index(&self, index: Position) -> &T {
        self.values.get(&index).unwrap()
    }
}

impl<T> IndexMut<Position> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn index_mut(&mut self, index: Position) -> &mut T {
        self.values.get_mut(&index).unwrap()
    }
}

impl<T> Index<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    type Output = T;

    fn index(&self, index: (Scalar, Scalar)) -> &T {
        self.values.get(&index.into()).unwrap()
    }
}

impl<T> IndexMut<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn index_mut(&mut self, index: (Scalar, Scalar)) -> &mut T {
        self.values.get_mut(&index.into()).unwrap()
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        self.values.get(&index.into()).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.values.get_mut(&index.into()).unwrap()
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Debug + Clone + Eq,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let width = self
            .values
            .values()
            .map(|v| format!("{}", v).len())
            .max()
            .unwrap_or(1);
        let filler = " ".repeat(width);
        for y in self.min_y..self.max_y {
            for x in self.min_x..self.max_x {
                if let Some(v) = self.get((x, y).into()) {
                    write!(f, "{:width$}", v, width = width)?;
                } else {
                    write!(f, "{}", filler)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
