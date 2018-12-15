#![allow(
dead_code,
unused_imports,
clippy::needless_range_loop,
clippy::ptr_arg,
clippy::char_lit_as_u8
)]

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
        use crate::dense_grid::Cardinal::*;
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
        use crate::dense_grid::Direction::*;
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
        use crate::dense_grid::Cardinal::*;
        use crate::dense_grid::Turn::*;
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
        use crate::dense_grid::Direction::*;
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
        use crate::dense_grid::Cardinal::*;
        use crate::dense_grid::Direction::*;
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
        use crate::dense_grid::Cardinal::*;
        use crate::dense_grid::Direction::*;
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
    T: Debug + Clone + Eq + Default,
{
    pub values: Vec<T>,
    pub min_x: Scalar,
    pub min_y: Scalar,
    pub max_x: Scalar,
    pub max_y: Scalar,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    pub fn new(
        min_x: Scalar,
        min_y: Scalar,
        max_x: Scalar,
        max_y: Scalar,
    ) -> Grid<T> {
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        let positions = width * height;
        let values =  vec![T::default(); positions];
        Grid {
            values,
            min_x,
            min_y,
            max_x,
            max_y,
            height,
            width,
        }
    }


    pub fn in_bounds(&self, position: Position) -> bool {
        self.min_x <= position.x && position.x <= self.max_x &&
        self.min_y <= position.y && position.y <= self.max_y
    }

    fn index(&self, position: Position) -> usize {
        if self.min_x == 0 && self.min_y == 0 {
            position.y as usize * self.width + position.x as usize
        } else {
           panic!("Unhandled case right now");
        }
    }

    pub fn insert(&mut self, position: Position, value: T) {
        if !self.in_bounds(position) {
            panic!(format!("Position {} is not in bounds {}, {}, {}, {}",
                           position, self.min_x, self.max_x, self.min_y, self.max_y));
        }
        let index = self.index(position);
        self.values[index] = value;
    }

    pub fn get<I>(&self, position: I) -> Option<&T>
    where I: Into<Position>
    {
        self.values.get(self.index(position.into()))
    }

    pub fn get_mut<I>(&mut self, position: I) -> Option<&mut T>
    where I: Into<Position>
    {
        let index = self.index(position.into());
        self.values.get_mut(index)
    }
}


impl<T> Index<Position> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    type Output = T;

    fn index(&self, position: Position) -> &T {
        self.get(position).unwrap()
    }
}

impl<T> IndexMut<Position> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    fn index_mut(&mut self, position: Position) -> &mut T {
        self.get_mut(position).unwrap()
    }
}

impl<T> Index<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    type Output = T;

    fn index(&self, position: (Scalar, Scalar)) -> &T {
        self.get(position).unwrap()
    }
}

impl<T> IndexMut<(Scalar, Scalar)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    fn index_mut(&mut self, position: (Scalar, Scalar)) -> &mut T {
        self.get_mut(position).unwrap()
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    type Output = T;

    fn index(&self, position: (usize, usize)) -> &T {
        self.get(position).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Debug + Clone + Eq + Default,
{
    fn index_mut(&mut self, position: (usize, usize)) -> &mut T {
        self.get_mut(position).unwrap()
    }
}

impl<T> Display for Grid<T>
where
    T: Display + Debug + Clone + Eq + Default,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let width = self
            .values
            .iter()
            .map(|v| format!("{}", v).len())
            .max()
            .unwrap_or(1);
        let filler = " ".repeat(width);
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if let Some(v) = self.get((x, y)) {
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
