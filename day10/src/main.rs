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

use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Input = Vec<Star>;
type Output = String;

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Star {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Star {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Star {
        Star { x, y, dx, dy }
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

impl FromStr for Star {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let numbers = get_numbers::<i32>(s)?;
        let (&x, &y, &dx, &dy) = numbers
            .iter()
            .next_tuple()
            .ok_or_else(|| err_msg("Not enough values"))?;
        Ok(Star::new(x, y, dx, dy))
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone, Debug)]
struct Sky {
    stars: Vec<Star>,
    second: usize,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

fn bounds(stars: &[Star]) -> (i32, i32, i32, i32) {
    let mut it = stars.iter();
    let first = it.next().unwrap();
    let mut min_x: i32 = first.x;
    let mut min_y: i32 = first.y;
    let mut max_x: i32 = first.x;
    let mut max_y: i32 = first.y;
    for star in it {
        min_x = min(min_x, star.x);
        min_y = min(min_y, star.y);
        max_x = max(max_x, star.x);
        max_y = max(max_y, star.y);
    }

    (min_x, min_y, max_x, max_y)
}

impl Sky {
    fn new(stars: &[Star]) -> Sky {
        let (min_x, min_y, max_x, max_y) = bounds(stars);
        Sky {
            stars: Vec::from(stars),
            second: 0,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn height(&self) -> usize {
        (self.max_y - self.min_y) as usize + 1
    }

    fn width(&self) -> usize {
        (self.max_x - self.min_x) as usize + 1
    }

    fn step(&mut self) {
        self.stars.iter_mut().for_each(Star::step);
        let (min_x, min_y, max_x, max_y) = bounds(&self.stars);
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
        self.second += 1;
    }
}

struct SkyIter {
    sky: Sky,
}

impl Iterator for SkyIter {
    type Item = Sky;

    fn next(&mut self) -> Option<Sky> {
        self.sky.step();
        Some(self.sky.clone())
    }
}

impl IntoIterator for Sky {
    type Item = Sky;
    type IntoIter = SkyIter;

    fn into_iter(self) -> SkyIter {
        SkyIter { sky: self }
    }
}

impl<'a> IntoIterator for &'a Sky {
    type Item = Sky;
    type IntoIter = SkyIter;

    fn into_iter(self) -> SkyIter {
        SkyIter { sky: self.clone() }
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut matrix = Matrix::new(self.width(), self.height());
        for star in self.stars.iter() {
            let x = (star.x - self.min_x) as usize;
            let y = (star.y - self.min_y) as usize;
            matrix[(x, y)] = true;
        }
        for y in 0..matrix.height {
            for x in 0..matrix.width {
                write!(f, "{}", if matrix[(x, y)] { '#' } else { '·' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?.trim().to_owned();
        if !line.is_empty() {
            result.push(line.parse()?)
        }
    }

    Ok(result)
}

fn find_message(input: &&mut Vec<Star>) -> Result<Sky, Error> {
    let sky = Sky::new(&input);
    let candidates = sky
        .into_iter()
        .skip_while(|sky| sky.height() > 100)
        .take_while(|sky| sky.height() <= 100);
    candidates
        .min_by_key(|s| s.height())
        .ok_or_else(|| err_msg("No message found"))
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let message = find_message(&input)?;
    Ok(message.to_string().to_owned())
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let message = find_message(&input)?;
    Ok(format!("{}", message.second))
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
