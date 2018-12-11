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
use im::vector::Vector;
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

type Input = (usize, usize);
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let numbers = get_numbers::<usize>(&line)?;
        if numbers.len() == 2 {
            return Ok((numbers[0], numbers[1]));
        }
    }

    bail!("Did not find any integers in input")
}

fn step(pos: usize, steps: i32, marbles: usize) -> usize {
    if steps >= 0 {
        (pos + (steps as usize)) % marbles
    } else {
        let steps = -steps as usize;
        if pos < steps {
            marbles - (steps - pos)
        } else {
            pos - steps
        }
    }
}

fn play(players: usize, max_score: usize) -> Vec<usize> {
    let mut scores = vec![0; players];
    let mut marbles = vec![0];
    let mut pos = 0;
    let mut player = 0;
    for marble in 1..=max_score {
        if marble % 23 == 0 {
            scores[player] += marble;
            pos = step(pos, -7, marbles.len());
            scores[player] += marbles[pos];
            marbles.remove(pos);
        } else {
            pos = step(pos, 1, marbles.len());
            marbles.insert(pos + 1, marble);
            pos += 1;
        }
        player = (player + 1) % players;
    }

    scores
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let (players, max_score) = *input;
    let scores = play(players, max_score);
    scores
        .into_iter()
        .max()
        .ok_or_else(|| err_msg("No scores?"))
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Marbles {
    head: Vec<usize>,
    tail: Vec<usize>,
}

impl Marbles {
    fn initial() -> Marbles {
        Marbles {
            head: vec![0],
            tail: Vec::new(),
        }
    }

    fn all_tail_to_head(&mut self) {
        while !self.tail.is_empty() {
            self.tail_to_head()
        }
    }

    fn tail_to_head(&mut self) {
        self.head.push(self.tail.pop().unwrap())
    }

    fn all_head_to_tail(&mut self) {
        while !self.head.is_empty() {
            self.head_to_tail()
        }
    }

    fn head_to_tail(&mut self) {
        self.tail.push(self.head.pop().unwrap())
    }

    fn step(&mut self, steps: i32) {
        if steps >= 0 {
            for _ in 0..steps {
                if self.tail.is_empty() {
                    self.all_head_to_tail();
                }
                self.tail_to_head();
            }
        } else {
            for _ in 0..steps.abs() {
                if self.head.is_empty() {
                    self.all_tail_to_head();
                }
                self.head_to_tail();
            }
        }
    }

    fn remove(&mut self) -> usize {
        let res = self.head.pop().unwrap();
        self.tail_to_head();
        res
    }

    fn insert(&mut self, value: usize) {
        self.head.push(value);
    }

    fn get(&self) -> usize {
        *self.head.last().unwrap()
    }

    fn len(&self) -> usize {
        self.head.len() + self.tail.len()
    }

    fn index(&self, i: usize) -> usize {
        if i < self.head.len() {
            self.head[i]
        } else {
            let i = i - self.head.len();
            self.tail[self.tail.len() - 1 - i]
        }
    }
}

impl Display for Marbles {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[")?;
        for m in self.head.iter() {
            write!(f, "{} ", m)?;
        }
        write!(f, "< ")?;
        for m in self.tail.iter().rev() {
            write!(f, "{} ", m)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

fn play2(players: usize, max_score: usize) -> Vec<usize> {
    let mut scores = vec![0; players];
    let mut marbles = Marbles::initial();
    let mut player = 0;
    for marble in 1..=max_score {
        if marble % 23 == 0 {
            scores[player] += marble;
            marbles.step(-7);
            scores[player] += marbles.remove();
        } else {
            marbles.step(1);
            marbles.insert(marble);
        }
        player = (player + 1) % players;
    }

    scores
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let (players, max_score) = *input;
    let scores = play2(players, 100 * max_score);
    scores
        .into_iter()
        .max()
        .ok_or_else(|| err_msg("No scores?"))
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
