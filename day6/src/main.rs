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
use rayon::prelude::*;
use stats::Frequencies;
use structopt::StructOpt;
use strum_macros::EnumString;
use tap::{TapOps, TapOptionOps, TapResultOps};

use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;

type Input = Vec<(usize, usize)>;
type Output = usize;

const PAD: usize = 2;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let numbers = get_numbers::<usize>(&line)?;
        if numbers.len() == 2 {
            result.push((numbers[0] + PAD, numbers[1] + PAD));
        }
    }

    Ok(result)
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let max_x = input.iter().map(|&(a, _)| a).max().unwrap() + 1 + PAD;
    let max_y = input.iter().map(|&(_, b)| b).max().unwrap() + 1 + PAD;

    let mut current = vec![vec![0 as usize; max_y]; max_x];
    let mut next = current.clone();

    for (i, &(x, y)) in input.iter().enumerate() {
        current[x][y] = i + 1;
    }
    print_matrix(max_x, max_y, &current);

    loop {
        let mut changed = false;
        for x in 1..(max_x - 1) {
            for y in 1..(max_y - 1) {
                let c = current[x][y];
                if c == 0 {
                    let u = current[x][y - 1];
                    let d = current[x][y + 1];
                    let l = current[x - 1][y];
                    let r = current[x + 1][y];
                    let n = [u, d, l, r]
                        .iter()
                        .filter(|&&v| v != 0)
                        .cloned()
                        .collect::<Vec<usize>>();
                    if n.is_empty() {
                        next[x][y] = 0;
                    } else {
                        let n1 = n[0];
                        if n.iter().filter(|&&v| v == n1).count() == n.len() {
                            next[x][y] = n1;
                            changed = true;
                        } else {
                            next[x][y] = 0;
                        }
                    }
                } else {
                    next[x][y] = c;
                }
            }
        }

        let tmp = current;
        current = next;
        next = tmp;

        print_matrix(max_x, max_y, &current);

        if !changed {
            break;
        }
    }

    print_matrix(max_x, max_y, &current);

    let mut infinite = HashSet::new();
    infinite.insert(0);
    for x in 0..max_x {
        infinite.insert(current[x][0]);
        infinite.insert(current[x][max_y - 1]);
    }
    for y in 0..max_y {
        infinite.insert(current[0][y]);
        infinite.insert(current[max_x - 1][y]);
    }

    let non_infinite = current
        .iter()
        .flatten()
        .filter(|v| !infinite.contains(v))
        .cloned()
        .collect::<Frequencies<usize>>();

    Ok(*non_infinite.most_frequent()[0].0)
}

fn print_matrix(max_x: usize, max_y: usize, current: &Vec<Vec<usize>>) {
    println!("Matrix:");
    for y in 1..max_y {
        for x in 1..max_x {
            if current[x][y] == 0 {
                print!("·");
            } else {
                let ch = (('a' as usize) + current[x][y] - 1) as u8 as char;
                print!("{}", ch);
            }
        }
        println!();
    }
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    Ok(2)
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
