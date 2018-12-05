#![allow(
    dead_code,
    unused_imports,
    clippy::needless_range_loop,
    clippy::ptr_arg,
    clippy::char_lit_as_u8
)]

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
use std::iter::*;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;

type Input = Vec<char>;
type Output = usize;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?.trim().to_owned();
        if !line.is_empty() {
            lines.push(line);
        }
    }
    assert_eq!(lines.len(), 1);

    let line = lines.pop().unwrap();
    let mut result = Vec::with_capacity(line.len() + 2);
    result.push('^');
    for ch in line.chars() {
        result.push(ch);
    }
    result.push('$');

    Ok(result)
}

fn polarity_pair(ch1: char, ch2: char) -> bool {
    ch1 != ch2 && ch1.to_lowercase().next() == ch2.to_lowercase().next()
}

fn reduce(input: &Input) -> Input {
    let mut result = Input::with_capacity(input.len());
    let mut iter = input.iter();
    result.push(*iter.next().unwrap());
    for &ch in iter {
        if polarity_pair(*result.last().unwrap(), ch) {
            result.pop();
        } else {
            result.push(ch);
        }
    }

    result
}

fn reduce_for(input: &Input, base: char) -> usize {
    let base_upper = base.to_uppercase().next().unwrap();
    let mut current = input
        .iter()
        .filter(|&&ch| ch != base && ch != base_upper)
        .cloned()
        .collect();
    loop {
        let next = reduce(&current);
        if current.len() == next.len() {
            return current.len() - 2;
        }
        current = next;
    }
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    (('a' as u8)..=('z' as u8))
        .map(|base| reduce_for(&input, base as char))
        .min()
        .ok_or_else(|| err_msg("No min found?"))
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
