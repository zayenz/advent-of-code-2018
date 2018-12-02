#![allow(dead_code, unused_imports)]

use failure::bail;
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

type Input = Vec<String>;
type Output = String;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        result.push(line.trim().to_owned())
    }

    Ok(result)
}

fn is_fabric_pair(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(c1, c2)| c1 != c2).count() == 1
}

fn common(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(a, _)| a)
        .collect()
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    for a in input.iter() {
        for b in input.iter() {
            if is_fabric_pair(a, b) {
                return Ok(common(a, b));
            }
        }
    }
    bail!("No fabric pair found")
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
