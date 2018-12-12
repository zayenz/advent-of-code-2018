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

type Input = (State, Rules);
type Output = i64;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
struct Pattern {
    data: [bool; 5],
}

impl Pattern {
    fn new(data: &[bool]) -> Pattern {
        assert_eq!(data.len(), 5);
        Pattern {
            data: [data[0], data[1], data[2], data[3], data[4]],
        }
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let trimmed = s.trim();
        let data: Vec<bool> = trimmed.chars().map(|c| c == '#').collect();
        if data.len() == 5 {
            Ok(Pattern::new(&data))
        } else {
            Err(err_msg(format!(
                "Input data \"{}\"not of length 5",
                trimmed
            )))
        }
    }
}

impl From<&[bool]> for Pattern {
    fn from(data: &[bool]) -> Self {
        Pattern::new(data)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
struct State {
    offset: i64,
    data: Vec<bool>,
}

impl State {
    fn initial(data: Vec<bool>) -> State {
        let mut data = data;
        for _ in 0..5 {
            data.insert(0, false);
            data.push(false);
        }
        State { offset: -5, data }
    }

    fn ensure_size(&mut self) {
        while self.data.iter().take(5).any(|&v| v) {
            self.data.insert(0, false);
            self.offset -= 1;
        }
        while self.data.iter().rev().take(5).any(|&v| v) {
            self.data.push(false);
        }
    }

    fn step(&mut self, rules: &Rules) {
        self.ensure_size();
        let mut next = vec![false; self.data.len()];
        for i in 2..self.data.len() - 2 {
            next[i] = rules.result(&self.data[i - 2..=i + 2].into());
        }
        self.data = next;
    }

    fn value(&self) -> i64 {
        self.data
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v)
            .map(|(i, _)| i as i64 + self.offset)
            .sum()
    }
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let trimmed = s.trim();
        let data = trimmed.chars().map(|c| c == '#').collect();
        Ok(State::initial(data))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for i in 0..self.data.len() {
            if i == self.offset.abs() as usize {
                write!(f, "|")?;
            }
            if self.data[i] {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
struct Rule {
    pattern: Pattern,
    result: bool,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let parts = s.split_whitespace().collect_vec();
        if parts.len() != 3 {
            return Err(err_msg(format!(
                "Rule \"{}\" does not contain three parts",
                s
            )));
        }
        Ok(Rule {
            pattern: parts[0].parse()?,
            result: parts[2].starts_with('#'),
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rules {
    rule: HashMap<Pattern, bool>,
}

impl Rules {
    fn new(rules: Vec<Rule>) -> Rules {
        let mut map = HashMap::with_capacity(rules.len());
        for Rule { pattern, result } in rules {
            map.insert(pattern, result);
        }
        Rules { rule: map }
    }

    fn result(&self, pattern: &Pattern) -> bool {
        self.rule.get(pattern).cloned().unwrap_or(false)
    }
}

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let state: State = lines
        .next()
        .ok_or_else(|| err_msg("No input?"))??
        .split_whitespace()
        .nth(2)
        .ok_or_else(|| err_msg("Insuficinet parts in first line"))?
        .parse()?;
    let rules = lines
        .filter_map(Result::ok)
        .filter(|s| !s.is_empty())
        .map(|l| l.parse())
        .collect::<Result<Vec<Rule>, _>>()?;
    let rules = Rules::new(rules);

    Ok((state, rules))
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let (state, rules) = input;

    for _ in 0..20 {
        state.step(rules);
    }

    Ok(state.value())
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let (state, rules) = input;

    const START: i64 = 1000;

    let mut current = state.clone();
    for _ in 0..START {
        current.step(rules);
    }
    let v1 = current.value();
    current.step(rules);
    let v2 = current.value();
    let diff = v2 - v1;

    Ok(v1 + (50_000_000_000 - START) * diff)
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

    println!("{:?}", output);
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
