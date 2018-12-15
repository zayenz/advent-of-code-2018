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

use aoc2018::dense_grid::Grid;
use aoc2018::dense_grid::*;
use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Input = Grid<Tile>;
type Output = String;

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Vitality {
    Alive,
    Dead,
}

use crate::Vitality::*;

impl Vitality {
    fn is_alive(self) -> bool {
        match self {
            Alive => true,
            Dead => false,
        }
    }

    fn is_dead(self) -> bool {
        !self.is_alive()
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Race {
    Elf,
    Goblin,
}

use crate::Race::*;

impl Race {
    fn enemy(self) -> Race {
        match self {
            Race::Elf => Race::Goblin,
            Race::Goblin => Race::Elf,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Creature {
    race: Race,
    hp: usize,
}

impl Creature {
    fn new(race: Race) -> Creature {
        Creature { race, hp: 200 }
    }

    fn race(&self) -> Race {
        self.race
    }

    fn enemy(&self) -> Race {
        self.race().enemy()
    }

    fn is_enemy(&self, race: Race) -> bool {
        self.enemy() == race
    }

    fn hp(&self) -> usize {
        self.hp
    }

    fn strength(&self) -> usize {
        3
    }

    fn take_damage(&mut self, damage: usize) -> Vitality {
        if damage >= self.hp {
            self.hp = 0;
            Dead
        } else {
            self.hp -= damage;
            Alive
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Floor,
    Occupied(Creature),
}

use crate::Tile::*;

impl Default for Tile {
    fn default() -> Self {
        Floor
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Floor => '.',
                Tile::Occupied(creature) => match creature.race() {
                    Race::Elf => 'E',
                    Race::Goblin => 'G',
                },
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
        .collect_vec();
    let height = lines.len();
    let width = lines[0].len();
    assert!(lines.iter().map(|v| v.len()).all_equal());
    let mut grid = Grid::new(0, 0, (width - 1) as i32, (height - 1) as i32);

    let lines = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();

    for x in 0..width {
        for y in 0..height {
            match lines[y][x] {
                '.' => {}
                '#' => {
                    grid[(x, y)] = Wall;
                }
                'E' => {
                    grid[(x, y)] = Occupied(Creature::new(Elf));
                }
                'G' => {
                    grid[(x, y)] = Occupied(Creature::new(Goblin));
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

    println!("Read initial map:");
    println!("{}", grid);

    Ok(grid)
}

fn solve1(_input: &mut Input) -> Result<Output, Error> {
    Ok("1".to_owned())
}

fn solve2(_input: &mut Input) -> Result<Output, Error> {
    Ok("2".to_owned())
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
