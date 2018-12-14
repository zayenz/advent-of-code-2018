#![allow(
    dead_code,
    unused_imports,
    clippy::needless_range_loop,
    clippy::ptr_arg,
    clippy::char_lit_as_u8
)]

use std::char;
use std::cmp::{max, min};
use std::fmt;
use std::io::BufRead;
use std::iter::*;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use failure::bail;
use failure::err_msg;
use failure::Error;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use stats::Frequencies;
use structopt::StructOpt;
use strum_macros::EnumString;
use tap::{TapOps, TapOptionOps, TapResultOps};

use aoc2018::grid::Step;
use aoc2018::grid::*;
use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Input = (Grid<Track>, Vec<Cart>);
type Output = Position;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[rustfmt::skip]
enum Track {
    Horizontal { location: Position, left: Position, right: Position, },
    Vertical { location: Position, up: Position, down: Position, },
    DLCorner { location: Position, down: Position, left: Position, },
    DRCorner { location: Position, down: Position, right: Position, },
    ULCorner { location: Position, up: Position, left: Position, },
    URCorner { location: Position, up: Position, right: Position, },
    Crossing { location: Position, up: Position, down: Position, left: Position, right: Position, },
}

use crate::Track::*;

#[rustfmt::skip]
impl Track {
    fn step(&self, direction: Direction) -> Option<Position> {
        match (self, direction) {
            (Horizontal { left, .. }, Direction::Left) => Some(*left),
            (Horizontal { right, .. }, Direction::Right) => Some(*right),
            (Vertical { up, .. }, Direction::Up) => Some(*up),
            (Vertical { down, .. }, Direction::Down) => Some(*down),
            (DLCorner { down, .. }, Direction::Down) => Some(*down),
            (DLCorner { left, .. }, Direction::Left) => Some(*left),
            (DRCorner { down, .. }, Direction::Down) => Some(*down),
            (DRCorner { right, .. }, Direction::Right) => Some(*right),
            (ULCorner { up, .. }, Direction::Up) => Some(*up),
            (ULCorner { left, .. }, Direction::Left) => Some(*left),
            (URCorner { up, .. }, Direction::Up) => Some(*up),
            (URCorner { right, .. }, Direction::Right) => Some(*right),
            (Crossing { up, .. }, Direction::Up) => Some(*up),
            (Crossing { down, .. }, Direction::Down) => Some(*down),
            (Crossing { left, .. }, Direction::Left) => Some(*left),
            (Crossing { right, .. }, Direction::Right) => Some(*right),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Cart {
    position: Position,
    direction: Direction,
    next_turn: Option<Turn>,
}

impl Cart {
    fn new(position: Position, direction: Direction) -> Cart {
        Cart {
            position,
            direction,
            next_turn: Some(Turn::Left),
        }
    }

    fn step(&self, grid: &Grid<Track>) -> Cart {
        use aoc2018::grid::Direction::*;

        let start_track = grid[self.position];
        let position = start_track
            .step(self.direction)
            .expect("Carts must always follow tracks");
        let track = grid[position];
        let (direction, next_turn) = match (track, self.direction) {
            (DLCorner { .. }, Right) => (Down, self.next_turn),
            (DLCorner { .. }, Up) => (Left, self.next_turn),
            (DRCorner { .. }, Left) => (Down, self.next_turn),
            (DRCorner { .. }, Up) => (Right, self.next_turn),
            (ULCorner { .. }, Right) => (Up, self.next_turn),
            (ULCorner { .. }, Down) => (Left, self.next_turn),
            (URCorner { .. }, Left) => (Up, self.next_turn),
            (URCorner { .. }, Down) => (Right, self.next_turn),
            (Crossing { .. }, _) => match self.next_turn {
                Some(Turn::Left) => (self.direction.turn(Turn::Left), None),
                None => (self.direction, Some(Turn::Right)),
                Some(Turn::Right) => (self.direction.turn(Turn::Right), Some(Turn::Left)),
            },
            _ => (self.direction, self.next_turn),
        };

        Cart {
            position,
            direction,
            next_turn,
        }
    }
}

#[rustfmt::skip]
fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let lines: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect_vec();

    let mut grid = Grid::new();
    let mut carts = Vec::new();

    // Get carts and transform carts to standard track markers
    let lines = lines.iter().enumerate()
        .map(|(y, row)| row.iter().enumerate()
            .map(|(x, &ch)|
                match ch {
                    '^' => {
                        carts.push(Cart::new((x, y).into(), Direction::Up));
                        '|'
                    }
                    'v' => {
                        carts.push(Cart::new((x, y).into(), Direction::Down));
                        '|'
                    }
                    '<' => {
                        carts.push(Cart::new((x, y).into(), Direction::Left));
                        '-'
                    }
                    '>' => {
                        carts.push(Cart::new((x, y).into(), Direction::Right));
                        '-'
                    }
                    _ => ch
                }
            ).collect_vec()
        ).collect_vec();

    let max_length = lines.iter().map(Vec::len).max().unwrap();
    let lines = lines.iter().cloned().map(|mut row| {
        while row.len() < max_length {
            row.push(' ');
        }
        row
    }).collect_vec();

    for (y, row) in lines.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let location: Position = (x, y).into();
            let up = location.step(Direction::Up);
            let down = location.step(Direction::Down);
            let left = location.step(Direction::Left);
            let right= location.step(Direction::Right);
            match ch {
                '|' => {grid.insert(location, Vertical { location, up, down });},
                '-' => {grid.insert(location, Horizontal { location, left, right });},
                '+' => {grid.insert(location, Crossing { location, up, down, left, right });},
                '/' => {
                    if y > 0 && (lines[y-1][x] == '|' || lines[y-1][x] == '+') {
                        grid.insert(location, ULCorner {location, up, left});
                    } else {
                        grid.insert(location, DRCorner {location, down, right});
                    }
                }
                '\\' => {
                    if y > 0 && (lines[y-1][x] == '|' || lines[y-1][x] == '+')  {
                        grid.insert(location, URCorner {location, up, right});
                    } else {
                        grid.insert(location, DLCorner {location, down, left})
                    }
                }
                ' ' => (),
                _ => bail!(format!("Found unexpected character '{}' at {}", ch, location)),
            }
        }
    }


    Ok((grid, carts))
}

fn print(grid: &Grid<Track>, carts: &[Cart]) {
    let mut cart_map = HashMap::new();
    carts.iter().for_each(|c| {
        cart_map.insert(c.position, c);
    });

    for y in grid.min_y..=grid.max_y {
        for x in grid.min_x..=grid.max_x {
            print!(
                "{}",
                if let Some(cart) = cart_map.get(&(x, y).into()) {
                    match cart.direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Right => '>',
                        Direction::Left => '<',
                    }
                } else if let Some(&track) = grid.get((x, y).into()) {
                    match track {
                        Track::Horizontal { .. } => '-',
                        Track::Vertical { .. } => '|',
                        Track::DLCorner { .. } => '\\',
                        Track::DRCorner { .. } => '/',
                        Track::ULCorner { .. } => '/',
                        Track::URCorner { .. } => '\\',
                        Track::Crossing { .. } => '+',
                    }
                } else {
                    ' '
                }
            );
        }
        println!();
    }
    println!("    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~    ");
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let (grid, carts) = input;
    let mut carts = carts.clone();

    loop {
        let mut positions = carts.iter().map(|&c| c.position).collect::<HashSet<_>>();
        carts.sort();
        let mut moved_carts = Vec::new();
        for cart in carts {
            positions.remove(&cart.position);
            let moved_cart = cart.step(&grid);
            if positions.contains(&moved_cart.position) {
                return Ok(moved_cart.position);
            }
            positions.insert(moved_cart.position);
            moved_carts.push(moved_cart);
        }
        carts = moved_carts;
    }
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let (grid, carts) = input;
    let mut carts = carts.clone();

    loop {
        let mut positions = carts
            .iter()
            .map(|&c| (c.position, c))
            .collect::<HashMap<_, _>>();

        carts.sort();
        for cart in carts {
            if positions.remove(&cart.position).is_some() {
                let moved_cart = cart.step(&grid);
                if positions.remove(&moved_cart.position).is_none() {
                    positions.insert(moved_cart.position, moved_cart);
                }
            }
        }
        if positions.len() == 1 {
            return Ok(*positions.keys().next().unwrap());
        }
        carts = positions.values().cloned().collect_vec();
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "day6")]
struct Opt {
    /// Part to solve, either 1 or 2
    #[structopt(short = "-p", long = "--part", default_value = "1")]
    part: u8,
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;

    let options: Opt = Opt::from_args();

    let output = if options.part == 1 {
        solve1(&mut input)?
    } else {
        solve2(&mut input)?
    };

    println!("{}", output);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            eprintln!("Error while solving problem: {}", error);
            for cause in error.iter_causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
