#![allow(dead_code, unused_imports, clippy::needless_range_loop)]

use failure::bail;
use failure::err_msg;
use failure::Error;
use rayon::prelude::*;
use strum_macros::EnumString;
use tap::{TapOps, TapOptionOps, TapResultOps};

use hashbrown::{HashMap, HashSet};
use std::char;
use std::cmp::{max, min};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use aoc2018::matrix::*;

type Input = Vec<Claim>;
type Output = i32;

struct Claim {
    id: i32,
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Claim {
    fn new(id: i32, left: usize, right: usize, top: usize, bottom: usize) -> Claim {
        Claim {
            id,
            left,
            right,
            top,
            bottom,
        }
    }

    fn width(&self) -> usize {
        self.right - self.left + 1
    }

    fn height(&self) -> usize {
        self.bottom - self.top + 1
    }
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cleaned: String = s
            .chars()
            .map(|c| {
                if c == '#' || c == 'x' || c == '@' || c == ':' || c == ',' {
                    ' '
                } else {
                    c
                }
            })
            .collect();
        let tokens = cleaned.split_whitespace().collect::<Vec<_>>();
        let id = tokens[0].parse()?;
        let left = tokens[1].parse()?;
        let top = tokens[2].parse()?;
        let width: usize = tokens[3].parse()?;
        let height: usize = tokens[4].parse()?;
        let right = left + width - 1;
        let bottom = top + height - 1;
        Ok(Claim::new(id, left, right, top, bottom))
    }
}

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        result.push(line.trim().parse()?)
    }

    Ok(result)
}

fn matching_ones(a: &Matrix, b: &Matrix) -> u32 {
    a.iter().zip(b.iter()).filter(|(a, b)| a == b).count() as u32
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    let mut claim_count = [[0; HEIGHT]; WIDTH];

    for claim in input.iter() {
        for w in claim.left..=claim.right {
            for h in claim.top..=claim.bottom {
                claim_count[w][h] += 1;
            }
        }
    }

    'claim_loop: for claim in input.iter() {
        for w in claim.left..=claim.right {
            for h in claim.top..=claim.bottom {
                if claim_count[w][h] > 1 {
                    continue 'claim_loop;
                }
            }
        }
        return Ok(claim.id);
    }

    bail!("No non-overlapping claim found")
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;
    let output = solve(&mut input)?;

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
