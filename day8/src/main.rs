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

type Input = Vec<usize>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        for n in get_numbers::<usize>(&line)? {
            result.push(n);
        }
    }

    Ok(result)
}

fn checksum(it: &mut impl Iterator<Item = usize>) -> usize {
    let mut result = 0;

    let child_nodes = it.next().unwrap();
    let checksum_entries = it.next().unwrap();

    for _child in 0..child_nodes {
        result += checksum(it);
    }

    for _entry in 0..checksum_entries {
        result += it.next().unwrap();
    }

    result
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let mut iter = input.iter().cloned();
    Ok(checksum(&mut iter))
}

fn node_value(it: &mut impl Iterator<Item = usize>) -> usize {
    let child_nodes = it.next().unwrap();
    let checksum_entries = it.next().unwrap();

    let mut child_values = Vec::new();
    for _child in 0..child_nodes {
        child_values.push(node_value(it));
    }

    let mut result = 0;
    for _entry_index in 0..checksum_entries {
        let entry = it.next().unwrap();
        if child_nodes > 0 {
            result += if entry <= child_nodes {
                child_values[entry - 1]
            } else {
                0
            };
        } else {
            result += entry;
        }
    }

    result
}
fn solve2(input: &mut Input) -> Result<Output, Error> {
    let mut iter = input.iter().cloned();
    Ok(node_value(&mut iter))
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
