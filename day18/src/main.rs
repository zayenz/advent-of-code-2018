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

use enum_map::*;
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

use aoc2018::dense_grid::Grid;
use aoc2018::dense_grid::*;
use aoc2018::input::*;
use aoc2018::matrix::*;
use aoc2018::position::*;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Input = Grid<Tile>;
type Output = String;

#[derive(EnumString, Enum, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Open,
    Trees,
    Lumberyard,
}

use crate::Tile::*;

impl Default for Tile {
    fn default() -> Self {
        Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                Empty => ' ',
                Open => '.',
                Trees => '|',
                Lumberyard => '#',
            }
        )
    }
}

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().trim().to_owned())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let height = lines.len();
    let width = lines[0].len();
    assert!(lines.iter().map(|v| v.len()).all_equal());
    let mut grid = Grid::from_origo(width + 2, height + 2);

    for x in 0..width {
        for y in 0..height {
            let pos = (x + 1, y + 1);
            match lines[y][x] {
                '.' => {
                    grid[pos] = Open;
                }
                '#' => {
                    grid[pos] = Lumberyard;
                }
                '|' => {
                    grid[pos] = Trees;
                }
                ch => {
                    bail!(format!(
                        "Unrecognized input charachter '{}' at {},{}",
                        ch, x, y
                    ));
                }
            }
        }
    }

    //    println!("Read initial map:");
    //    println!("{}", grid);

    Ok(grid)
}

fn count(it: &mut impl Iterator<Item = Tile>) -> EnumMap<Tile, usize> {
    let mut counts = EnumMap::new();
    for value in it {
        counts[value] += 1;
    }
    counts
}

fn checksum(grid: &Grid<Tile>) -> usize {
    let frequencies = count(&mut grid.values.iter().cloned());
    frequencies[Trees] * frequencies[Lumberyard]
}

fn evolve(input: &Grid<Tile>, iterations: usize) -> Grid<Tile> {
    let mut current = input.clone();

    for _ in 0..iterations {
        let mut next = current.clone();
        for x in 1..(input.width - 1) {
            for y in 1..(input.height - 1) {
                let pos: Position = (x, y).into();
                let tile = current[pos];
                let neighbours = count(&mut connect8(pos).map(|n| current[n]));
                next[pos] = match tile {
                    Empty => Empty,
                    Open => {
                        if neighbours[Trees] >= 3 {
                            Trees
                        } else {
                            Open
                        }
                    }
                    Trees => {
                        if neighbours[Lumberyard] >= 3 {
                            Lumberyard
                        } else {
                            Trees
                        }
                    }
                    Lumberyard => {
                        if neighbours[Lumberyard] >= 1 && neighbours[Trees] >= 1 {
                            Lumberyard
                        } else {
                            Open
                        }
                    }
                }
            }
        }

        current = next;
    }

    current
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let result = evolve(input, 10);
    Ok(format!("{}", checksum(&result)))
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let result = evolve(input, 1000);
    Ok(format!("{}", checksum(&result)))
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
