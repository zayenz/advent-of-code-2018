#![allow(dead_code, unused_imports)]

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
type Output = u32;

struct Claim {
    id: i32,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Claim {
    fn new(id: i32, left: i32, right: i32, top: i32, bottom: i32) -> Claim {
        Claim {
            id,
            left,
            right,
            top,
            bottom,
        }
    }

    fn width(&self) -> usize {
        (self.right - self.left + 1) as usize
    }

    fn height(&self) -> usize {
        (self.bottom - self.top + 1) as usize
    }

    fn to_matrix(&self, width: usize, height: usize) -> Matrix {
        let mut result = Matrix::new(width, height);
        self.mark(&mut result);
        result
    }

    fn mark(&self, matrix: &mut Matrix) {
        matrix.fill_true(
            self.left as usize,
            self.top as usize,
            self.width(),
            self.height(),
        );
    }
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cleaned: String = s
            .chars()
            .map(|c| {
                if c == '#' || c == 'x' || c == '@' || c == ':' {
                    ' '
                } else {
                    c
                }
            })
            .collect();
        let tokens = cleaned
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let id = tokens[0].parse()?;
        let coordinates = tokens[1].split(',').collect::<Vec<&str>>();
        let left = coordinates[0].parse()?;
        let top = coordinates[1].parse()?;
        let width: i32 = tokens[2].parse()?;
        let height: i32 = tokens[3].parse()?;
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
        result.push(line.trim().to_owned().parse()?)
    }

    Ok(result)
}

fn matching_ones(a: &Matrix, b: &Matrix) -> u32 {
    a.iter().zip(b.iter()).filter(|(a, b)| a == b).count() as u32
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    let max_right = input
        .iter()
        .map(|c| c.right)
        .max()
        .ok_or_else(|| err_msg("No input"))?;
    let max_bottom = input
        .iter()
        .map(|c| c.bottom)
        .max()
        .ok_or_else(|| err_msg("No input"))?;
    let width = (max_right + 2) as usize;
    let height = (max_bottom + 2) as usize;

    let mut claim_count = HashMap::new();
    for w in 0i32..(width as i32) {
        for h in 0i32..(height as i32) {
            claim_count.insert((w, h), 0);
        }
    }

    for claim in input.iter() {
        for w in claim.left..=claim.right {
            for h in claim.top..=claim.bottom {
                *claim_count.get_mut(&(w, h)).unwrap() += 1;
            }
        }
    }

    let mut result = 0;
    for w in 0i32..(width as i32) {
        for h in 0i32..(height as i32) {
            if claim_count[&(w, h)] > 1 {
                result += 1;
            }
        }
    }

    Ok(result)
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
