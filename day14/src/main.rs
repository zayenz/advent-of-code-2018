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

type Input = String;
type Output = String;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?.trim().to_owned();
        if !line.is_empty() {
            return Ok(line);
        }
    }

    bail!("No input found")
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Ledger {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
    iterations: usize,
}

impl Ledger {
    fn new() -> Ledger {
        Ledger {
            recipes: vec![3, 7],
            elf1: 0,
            elf2: 1,
            iterations: 0,
        }
    }

    fn next_pos(&self, elf: usize) -> usize {
        (elf + self.recipes[elf] as usize + 1) % self.recipes.len()
    }

    fn cook(&mut self) {
        let sum = self.recipes[self.elf1] + self.recipes[self.elf2];
        if sum >= 10 {
            self.recipes.push(sum / 10);
        }
        self.recipes.push(sum % 10);
        self.elf1 = self.next_pos(self.elf1);
        self.elf2 = self.next_pos(self.elf2);
        self.iterations += 1;
    }
}

impl Display for Ledger {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}: ", self.iterations)?;
        for i in 0..self.recipes.len() {
            if i == self.elf1 {
                write!(f, "(")?;
            }
            if i == self.elf2 {
                write!(f, "[")?;
            }

            write!(f, "{}", self.recipes[i])?;

            if i == self.elf1 {
                write!(f, ")")?;
            }
            if i == self.elf2 {
                write!(f, "]")?;
            }
            write!(f, " ")?;
        }
        Ok(())
    }
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let input: usize = input.parse()?;
    let mut ledger = Ledger::new();

    for _ in 0..input + 10 {
        ledger.cook();
    }

    let next10 = ledger.recipes[input..input + 10]
        .iter()
        .map(|&v| format!("{}", v))
        .collect::<String>();

    Ok(next10)
}

fn contains_at(data: &[u8], pos: usize, pattern: &[u8]) -> bool {
    for i in 0..pattern.len() {
        if data[pos + i] != pattern[i] {
            return false;
        }
    }
    true
}

fn position_of(data: &[u8], pattern: &[u8]) -> Option<usize> {
    if data.len() < pattern.len() {
        return None;
    }
    for i in 0..=data.len() - pattern.len() {
        if contains_at(data, i, pattern) {
            return Some(i);
        }
    }
    None
}

fn contains(data: &[u8], pattern: &[u8]) -> bool {
    position_of(data, pattern).is_some()
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let pattern = input
        .chars()
        .map(|c| c.to_string().parse())
        .collect::<Result<Vec<u8>, _>>()?;

    let mut ledger = Ledger::new();
    while ledger.recipes.len() < pattern.len() + 2 {
        ledger.cook();
    }

    while !contains(
        &ledger.recipes[(ledger.recipes.len() - pattern.len() - 2)..],
        &pattern,
    ) {
        ledger.cook();
    }

    Ok(format!(
        "{}",
        position_of(&ledger.recipes, &pattern).unwrap()
    ))
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
